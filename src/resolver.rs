use std::collections::HashMap;

use crate::{errors::LoxError, stmt::Stmt, token::Token};

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
}

pub type ResolutionMap = HashMap<Token, usize>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResolutionError {}

pub type ResolutionResult<T> = Result<T, ResolutionError>;

impl Resolver {
    #[must_use]
    fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    pub fn visit_statement(&mut self, stmt: Stmt) -> ResolutionResult<()> {
        match stmt {
            Stmt::Block(statements) => {
                self.begin_scope();
                self.resolve(statements);
                self.end_scope();
                Ok(())
            }
            _ => {
                unimplemented!()
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}
