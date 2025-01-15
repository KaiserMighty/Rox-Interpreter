use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token
{
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token
{
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self
    {
        Token
        {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!
        (
            f,
            "{} {} {} (line: {})",
            format!("{:?}", self.token_type),
            self.lexeme,
            self.literal.as_deref().unwrap_or("null"),
            self.line
        )
    }
}
