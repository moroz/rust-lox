use crate::interpreter::EvaluationResult;
use crate::literal::Literal;
use crate::token::Token;
use std::fmt::Display;

#[derive(Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
}

#[derive(Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
}

impl Stmt {
    pub fn evaluate(&self) -> EvaluationResult {
        match self {
            Self::Print(expr) => self.evaluate_print(expr),
            Self::Expression(expr) => expr.evaluate(),
        }
    }

    fn evaluate_print(&self, expr: &Expr) -> EvaluationResult {
        match expr.evaluate() {
            Ok(value) => {
                println!("{}", value);
                Ok(Literal::Nil)
            }
            other => other,
        }
    }
}

impl Display for Expr {
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
        let expr = Expr::Grouping(Box::new(Expr::Literal(Literal::Number(45.67))));

        let actual = format!("{}", expr);
        assert_eq!("(group 45.67)", actual);
    }

    #[test]
    fn test_serialize_unary() {
        let expr = Expr::Unary(
            Token::new(TokenType::Minus, "-".to_string(), 1),
            Box::new(Expr::Literal(Literal::Number(45.67))),
        );

        let actual = format!("{}", expr);
        assert_eq!("(- 45.67)", actual);
    }

    #[test]
    fn test_serialize_binary() {
        let left = Expr::Unary(
            Token::new(TokenType::Minus, "-".to_string(), 1),
            Box::new(Expr::Literal(Literal::Number(123.0))),
        );

        let right = Expr::Grouping(Box::new(Expr::Literal(Literal::Number(45.67))));

        let operator = Token::new(TokenType::Star, "*".to_string(), 1);

        let expr = Expr::Binary(Box::new(left), operator, Box::new(right));

        let actual = format!("{}", expr);
        assert_eq!("(* (- 123) (group 45.67))", actual);
    }
}
