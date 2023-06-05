use std::fmt::Display;

#[derive(Clone, PartialEq)]
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
