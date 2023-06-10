use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    interpreter::{EvaluationResult, Interpreter},
    literal::Literal,
    stmt::Stmt,
    token::Token,
};

#[derive(Clone)]
pub enum Function {
    Native {
        arity: usize,
        body: Box<fn(&Vec<Literal>) -> Literal>,
    },
    Lox {
        arity: usize,
        params: Box<Vec<Token>>,
        body: Box<Vec<Stmt>>,
    },
}

impl Function {
    pub fn arity(&self) -> usize {
        match self {
            Self::Native { arity, .. } => arity.clone(),
            Self::Lox { arity, .. } => arity.clone(),
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &Vec<Literal>,
    ) -> EvaluationResult {
        match self {
            Self::Native { body, .. } => Ok(body(arguments)),
            Self::Lox { body, params, .. } => {
                let mut env = Environment::enclose(&interpreter.globals);
                let mut i = 0;
                for param in params.iter() {
                    let value = arguments.get(i).unwrap();
                    env.define(param.lexeme.clone(), value.clone());
                    i += 1;
                }
                interpreter.execute_block(body, Rc::new(RefCell::new(env)))
            }
        }
    }
}
