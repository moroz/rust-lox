use crate::token::Token;
use std::fmt::Display;

#[derive(Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => {
                write!(f, "{}", s)
            }
            Self::Number(n) => {
                write!(f, "{}", n)
            }
            Self::Boolean(b) => {
                write!(f, "{}", b)
            }
            Self::Nil => {
                write!(f, "nil")
            }
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Grouping(Box<Expression>),
    Literal(Literal),
    Unary(Token, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Self::Grouping(expr) => {
                write!(f, "(group {})", expr)
            }
            Self::Literal(expr) => {
                write!(f, "{}", expr)
            }
            Self::Unary(operator, expr) => {
                write!(f, "({} {})", operator.lexeme, expr)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_serialize_grouping() {
        let expr = Expression::Grouping(Box::new(Expression::Literal(Literal::Number(45.67))));

        let actual = format!("{}", expr);
        assert_eq!("(group 45.67)", actual);
    }

    #[test]
    fn test_serialize_unary() {
        let expr = Expression::Unary(
            Token::new(TokenType::Minus, "-".to_string(), 1),
            Box::new(Expression::Literal(Literal::Number(45.67))),
        );

        let actual = format!("{}", expr);
        assert_eq!("(- 45.67)", actual);
    }

    #[test]
    fn test_serialize_binary() {
        let left = Expression::Unary(
            Token::new(TokenType::Minus, "-".to_string(), 1),
            Box::new(Expression::Literal(Literal::Number(123.0))),
        );

        let right = Expression::Grouping(Box::new(Expression::Literal(Literal::Number(45.67))));

        let operator = Token::new(TokenType::Star, "*".to_string(), 1);

        let expr = Expression::Binary(Box::new(left), operator, Box::new(right));

        let actual = format!("{}", expr);
        assert_eq!("(* (- 123) (group 45.67))", actual);
    }
}
