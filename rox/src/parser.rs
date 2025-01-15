use crate::token::{Token};
use crate::token_type::TokenType;
use crate::expr::{Expr};

#[derive(Debug)]
struct ParseError;

pub struct Parser<'a>
{
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a>
{
    pub fn new(tokens: &'a [Token]) -> Self
    {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr>
    {
        self.expression().ok()
    }

    fn expression(&mut self) -> Result<Expr, ParseError>
    {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError>
    {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual])
        {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError>
    {
        let mut expr = self.term()?;
        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual])
        {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError>
    {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus])
        {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError>
    {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::Slash, TokenType::Star])
        {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError>
    {
        if self.match_token(&[TokenType::Bang, TokenType::Minus])
        {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError>
    {
        if self.match_token(&[TokenType::False])
        {
            return Ok(Expr::Literal { value: Some("false".to_string()) });
        }
        if self.match_token(&[TokenType::True])
        {
            return Ok(Expr::Literal { value: Some("true".to_string()) });
        }
        if self.match_token(&[TokenType::Nil])
        {
            return Ok(Expr::Literal { value: None });
        }

        if self.match_token(&[TokenType::Number, TokenType::String])
        {
            return Ok(Expr::Literal { value: self.previous().literal.clone() });
        }

        if self.match_token(&[TokenType::LeftParen])
        {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool
    {
        for &token_type in types
        {
            if self.check(token_type)
            {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError>
    {
        if self.check(token_type)
        {
            Ok(self.advance())
        }
        else
        {
            Err(self.error(self.peek(), message))
        }
    }

    fn check(&self, token_type: TokenType) -> bool
    {
        if self.is_at_end()
        {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token
    {
        if !self.is_at_end()
        {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn is_at_end(&self) -> bool
    {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token
    {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token
    {
        &self.tokens[self.current - 1]
    }

    fn error(&self, token: &Token, message: &str) -> ParseError
    {
        eprintln!("Error at {}: {}", token.line, message);
        ParseError
    }

    fn synchronize(&mut self)
    {
        self.advance();

        while !self.is_at_end()
        {
            if self.previous().token_type == TokenType::Semicolon
            {
                return;
            }

            match self.peek().token_type
            {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For |
                TokenType::If | TokenType::While | TokenType::Print | TokenType::Return =>
                {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }
}
