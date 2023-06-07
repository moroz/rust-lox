use crate::{
    interpreter::{EvaluationResult, Interpreter},
    literal::Literal,
};

#[derive(PartialEq, Clone)]
pub enum Function {
    Native {
        arity: usize,
        body: Box<fn(&Vec<Literal>) -> Literal>,
    },
}

impl Function {
    pub fn arity(&self) -> usize {
        match self {
            Self::Native { arity, .. } => arity.clone(),
        }
    }

    pub fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &Vec<Literal>,
    ) -> EvaluationResult {
        match self {
            Self::Native { body, .. } => Ok(body(arguments)),
        }
    }
}
