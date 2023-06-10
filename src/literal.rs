use std::fmt::{Debug, Display};

use crate::function::Function;

#[derive(Clone)]
pub enum Literal {
    Function(Function),
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl PartialEq for Literal {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::String(lhs), Self::String(rhs)) => lhs == rhs,
            (Self::Number(lhs), Self::Number(rhs)) => lhs == rhs,
            (Self::Boolean(lhs), Self::Boolean(rhs)) => lhs == rhs,
            (Self::Nil, Self::Nil) => true,
            (_, _) => false,
        }
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => {
                write!(f, "\"{}\"", s)
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
            Self::Function(_) => {
                write!(f, "<native fn>")
            }
        }
    }
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
            Self::Function(_) => {
                write!(f, "<native fn>")
            }
        }
    }
}

impl Literal {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Boolean(false) => false,
            _ => true,
        }
    }
}
