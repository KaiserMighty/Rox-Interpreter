use crate::token::Token;

#[derive(Debug)]
pub struct RuntimeError
{
    pub token: Token,
    pub message: String,
}

impl RuntimeError
{
    pub fn new(token: Token, message: impl Into<String>) -> Self
    {
        RuntimeError
        {
            token,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for RuntimeError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Runtime error at {}: {}", self.token.lexeme, self.message)
    }
}

impl std::error::Error for RuntimeError {}
