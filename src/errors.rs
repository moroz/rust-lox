use crate::{literal::Literal, token::Token};

#[derive(Clone, Debug, PartialEq)]
pub enum LoxErrorType {
    SyntaxError(String),
    RuntimeError(DetailedErrorType),
    Return(Literal),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DetailedErrorType {
    ExpectedNumber,
    UndeclaredIdentifier,
    InvalidArity,
    NotCallable,
}

#[derive(Clone, Debug)]
pub struct LoxError {
    pub token: Token,
    pub kind: LoxErrorType,
    pub line: usize,
}

impl LoxError {
    pub fn new(token: &Token, kind: LoxErrorType) -> Self {
        Self {
            line: token.line,
            kind,
            token: token.clone(),
        }
    }

    pub fn parse_error(token: &Token, msg: impl Into<String>) -> Self {
        Self {
            line: token.line,
            kind: LoxErrorType::SyntaxError(msg.into()),
            token: token.clone(),
        }
    }
}
