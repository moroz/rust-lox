use std::fmt::{Debug, Display};

use crate::function::Function;

#[derive(Clone, PartialEq)]
pub enum Literal {
    Function(Function),
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
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
