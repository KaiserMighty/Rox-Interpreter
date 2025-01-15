use crate::expr::{Expr, Visitor};
use crate::token::{Token};
use crate::token_type::TokenType;
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::Rox;

pub struct Interpreter;

impl Interpreter
{
    pub fn new() -> Self
    {
        Interpreter
    }

    pub fn interpret(&mut self, expression: Expr, rox: &mut Rox)
    {
        match self.evaluate(&expression)
        {
            Ok(value) => println!("{}", self.stringify(&value)),
            Err(error) => rox.runtime_error(&error),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, RuntimeError>
    {
        expr.accept(self)
    }

    fn check_number_operand(&self, operator: &Token, operand: &Object) -> Result<(), RuntimeError>
    {
        if matches!(operand, Object::Number(_))
        {
            Ok(())
        }
        else
        {
            Err(RuntimeError::new(operator.clone(), "Operand must be a number.".to_string()))
        }
    }

    fn check_number_operands(&self, operator: &Token, left: &Object, right: &Object) -> Result<(), RuntimeError>
    {
        if matches!(left, Object::Number(_)) && matches!(right, Object::Number(_))
        {
            Ok(())
        }
        else
        {
            Err(RuntimeError::new(operator.clone(), "Operands must be numbers.".to_string()))
        }
    }

    fn is_truthy(&self, object: &Object) -> bool
    {
        match object
        {
            Object::Nil => false,
            Object::Boolean(value) => *value,
            _ => true,
        }
    }

    fn is_equal(&self, a: &Object, b: &Object) -> bool
    {
        a == b
    }

    fn stringify(&self, object: &Object) -> String
    {
        match object
        {
            Object::Nil => "nil".to_string(),
            Object::Number(num) =>
            {
                let mut s = num.to_string();
                if s.ends_with(".0")
                {
                    s.truncate(s.len() - 2);
                }
                s
            }
            _ => object.to_string(),
        }
    }
}

impl Visitor<Result<Object, RuntimeError>> for Interpreter
{
    fn visit_literal_expr(&mut self, value: &Option<String>) -> Result<Object, RuntimeError>
    {
        match value
        {
            Some(string_value) =>
            {
                if let Ok(num) = string_value.parse::<f64>()
                {
                    Ok(Object::Number(num))
                }
                else
                {
                    Ok(Object::String(string_value.clone()))
                }
            }
            None => Ok(Object::Nil),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, RuntimeError>
    {
        let right = self.evaluate(right)?;

        match operator.token_type
        {
            TokenType::Bang => Ok(Object::Boolean(!self.is_truthy(&right))),
            TokenType::Minus =>
            {
                self.check_number_operand(operator, &right)?;
                if let Object::Number(value) = right
                {
                    Ok(Object::Number(-value))
                }
                else
                {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Object, RuntimeError>
    {
        self.evaluate(expression)
    }

    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object, RuntimeError>
    {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type
        {
            TokenType::Greater =>
            {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(left), Object::Number(right)) = (left, right)
                {
                    Ok(Object::Boolean(left > right))
                }
                else
                {
                    unreachable!()
                }
            }
            TokenType::GreaterEqual =>
            {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(left), Object::Number(right)) = (left, right)
                {
                    Ok(Object::Boolean(left >= right))
                }
                else
                {
                    unreachable!()
                }
            }
            TokenType::Less => {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(left), Object::Number(right)) = (left, right)
                {
                    Ok(Object::Boolean(left < right))
                }
                else
                {
                    unreachable!()
                }
            }
            TokenType::LessEqual =>
            {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(left), Object::Number(right)) = (left, right)
                {
                    Ok(Object::Boolean(left <= right))
                }
                else
                {
                    unreachable!()
                }
            }
            TokenType::BangEqual => Ok(Object::Boolean(!self.is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(Object::Boolean(self.is_equal(&left, &right))),
            TokenType::Minus =>
            {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(left), Object::Number(right)) = (left, right)
                {
                    Ok(Object::Number(left - right))
                }
                else
                {
                    unreachable!()
                }
            }
            TokenType::Plus =>
            {
                if let (Object::Number(left), Object::Number(right)) = (&left, &right)
                {
                    return Ok(Object::Number(left + right));
                }
                if let (Object::String(left), Object::String(right)) = (&left, &right)
                {
                    return Ok(Object::String(left.clone() + right));
                }
                Err(RuntimeError::new(operator.clone(), "Operands must be two numbers or two strings.".to_string()))
            }
            TokenType::Slash =>
            {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(left), Object::Number(right)) = (left, right)
                {
                    Ok(Object::Number(left / right))
                }
                else
                {
                    unreachable!()
                }
            }
            TokenType::Star =>
            {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(left), Object::Number(right)) = (left, right)
                {
                    Ok(Object::Number(left * right))
                }
                else
                {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}
