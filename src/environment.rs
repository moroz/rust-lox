use crate::literal::Literal;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Literal>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn assign(&mut self, name: impl Into<String>, value: Literal) -> bool {
        let name: String = name.into();
        match self.values.get(&name) {
            Some(_) => {
                self.values.insert(name.into(), value);
                true
            }
            None => match &mut self.enclosing {
                None => false,
                Some(env) => env.assign(name, value),
            },
        }
    }

    pub fn define(&mut self, name: impl Into<String>, value: Literal) {
        self.values.insert(name.into(), value);
    }

    pub fn fetch(&self, name: impl Into<String>) -> Option<&Literal> {
        let name: String = name.into();
        match self.values.get(&name) {
            Some(value) => Some(value),
            None => match &self.enclosing {
                None => None,
                Some(env) => env.fetch(name),
            },
        }
    }
}
