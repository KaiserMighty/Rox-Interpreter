use crate::expr::{Expr, Visitor};
use crate::token::Token;

pub struct AstPrinter;

impl AstPrinter
{
    pub fn print(&self, expr: &Expr) -> String
    {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String
    {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs
        {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');
        builder
    }
}

impl Visitor<String> for AstPrinter
{
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String
    {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> String
    {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal_expr(&mut self, value: &Option<String>) -> String
    {
        match value {
            Some(v) => v.clone(),
            None => "nil".to_string(),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String
    {
        self.parenthesize(&operator.lexeme, &[right])
    }
}
