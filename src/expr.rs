use crate::literal::Literal;
use crate::token::Token;
use std::fmt::Debug;

#[derive(Clone)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Var(Token),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(left, operator, right) => {
                write!(f, "({} {:?} {:?})", operator.lexeme, left, right)
            }
            Self::Grouping(expr) => {
                write!(f, "(group {:?})", expr)
            }
            Self::Literal(expr) => {
                write!(f, "{}", expr)
            }
            Self::Unary(operator, expr) => {
                write!(f, "({} {:?})", operator.lexeme, expr)
            }
            Self::Var(token) => {
                write!(f, "(var {})", token.lexeme)
            }
            Self::Assign(token, expr) => {
                write!(f, "(assign {} {:?})", token.lexeme, expr)
            }
            Self::Logical(left, operator, right) => {
                write!(f, "({} {:?} {:?})", operator.lexeme, left, right)
            }
            Self::Call(callee, _, arguments) => {
                let args: Vec<_> = arguments.iter().map(|arg| format!("{:?}", arg)).collect();
                let args = args.join(" ");
                write!(f, "({:?} {:?})", callee, args)
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

        let actual = format!("{:?}", expr);
        assert_eq!("(group 45.67)", actual);
    }

    #[test]
    fn test_serialize_unary() {
        let expr = Expr::Unary(
            Token::new(TokenType::Minus, "-".to_string(), 1),
            Box::new(Expr::Literal(Literal::Number(45.67))),
        );

        let actual = format!("{:?}", expr);
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

        let actual = format!("{:?}", expr);
        assert_eq!("(* (- 123) (group 45.67))", actual);
    }
}
