use crate::token::Token;

pub enum Expr
{
    Binary
    {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping
    {
        expression: Box<Expr>,
    },
    Literal
    {
        value: Option<String>,
    },
    Unary
    {
        operator: Token,
        right: Box<Expr>,
    },
}

pub trait Visitor<R>
{
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> R;
    fn visit_literal_expr(&mut self, value: &Option<String>) -> R;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> R;
}

impl Expr
{
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R
    {
        match self
        {
            Expr::Binary { left, operator, right } =>
            {
                visitor.visit_binary_expr(left, operator, right)
            }
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}