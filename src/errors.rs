use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum LoxErrorType {
    SyntaxError,
    RuntimeError,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DetailedErrorType {
    ExpectedNumber,
    ExpectedNumberOrString,
    UndeclaredIdentifier,
}

#[derive(Clone, Debug)]
pub struct LoxError {
    token: Token,
    kind: LoxErrorType,
    detailed_type: DetailedErrorType,
    line: usize,
    msg: Option<String>,
}

impl LoxError {
    pub fn new(token: &Token, kind: LoxErrorType, detailed_type: DetailedErrorType) -> Self {
        Self {
            line: token.line,
            kind,
            detailed_type,
            token: token.clone(),
            msg: None,
        }
    }
}
