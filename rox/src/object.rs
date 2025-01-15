use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Object
{
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Object
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "{}", s),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Nil => write!(f, "nil"),
        }
    }
}
