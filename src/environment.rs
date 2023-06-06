use crate::literal::Literal;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: impl Into<String>, value: Literal) {
        self.values.insert(name.into(), value);
    }

    pub fn fetch(&mut self, name: impl Into<String>) -> Option<&Literal> {
        self.values.get(&name.into())
    }
}
