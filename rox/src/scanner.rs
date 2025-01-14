use crate::Rox;
use crate::token::Token;
use crate::token_type::TokenType;
use std::collections::HashMap;

pub struct Scanner<'a>
{
    source: String,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    start: usize,
    current: usize,
    line: usize,
    rox: &'a mut Rox,
}

impl <'a> Scanner<'a>
{
    pub fn new(source: String, rox: &'a mut Rox) -> Self 
    {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);

        Scanner
        {
            source,
            tokens: Vec::new(),
            keywords,
            start: 0,
            current: 0,
            line: 1,
            rox,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token>
    {
        while !self.is_at_end()
        {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        self.tokens.clone()
    }

    fn scan_token(&mut self)
    {
        let c = self.advance();
        match c
        {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' =>
            {
                let token_type = if self.match_char('=')
                {
                    TokenType::BangEqual
                }
                else
                {
                    TokenType::Bang
                };
                self.add_token(token_type, None);
            }
            '=' =>
            {
                let token_type = if self.match_char('=')
                {
                    TokenType::EqualEqual
                }
                else
                {
                    TokenType::Equal
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.match_char('=')
                {
                    TokenType::LessEqual
                }
                else
                {
                    TokenType::Less
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.match_char('=')
                {
                    TokenType::GreaterEqual
                }
                else
                {
                    TokenType::Greater
                };
                self.add_token(token_type, None);
            }
            '/' =>
            {
                if self.match_char('/')
                {
                    while self.peek() != '\n' && !self.is_at_end()
                    {
                        self.advance();
                    }
                }
                else
                {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ =>
            {
                if c.is_ascii_digit()
                {
                    self.number();
                }
                else if c.is_ascii_alphabetic() || c == '_'
                {
                    self.identifier();
                }
                else
                {
                    self.rox.error(self.line, "Unexpected character.");
                }
            }
        }
    }

    fn identifier(&mut self)
    {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_'
        {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = self.keywords.get(text).cloned().unwrap_or(TokenType::Identifier);
        self.add_token(token_type, None);
    }

    fn number(&mut self)
    {
        while self.peek().is_ascii_digit()
        {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit()
        {
            self.advance();
            while self.peek().is_ascii_digit()
            {
                self.advance();
            }
        }
        let value = &self.source[self.start..self.current];
        self.add_token(TokenType::Number, Some(value.to_string()));
    }

    fn string(&mut self)
    {
        while self.peek() != '"' && !self.is_at_end()
        {
            if self.peek() == '\n'
            {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end()
        {
            self.rox.error(self.line, "Unterminated string.");
            return;
        }
        self.advance(); // Closing "
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(value.to_string()));
    }

    fn match_char(&mut self, expected: char) -> bool
    {
        if self.is_at_end() || self.source.chars().nth(self.current) != Some(expected)
        {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char
    {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char
    {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool
    {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char
    {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>)
    {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text.to_string(), literal, self.line));
    }
}
