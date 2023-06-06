use crate::literal::Literal;
use std::{cell::RefCell, collections::HashMap};

pub struct Environment<'a> {
    values: HashMap<String, Literal>,
    pub enclosing: Option<&'a RefCell<Environment<'a>>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn enclosed(enclosing: &'a RefCell<Self>) -> RefCell<Self> {
        RefCell::new(Self {
            values: HashMap::new(),
            enclosing: Some(&enclosing),
        })
    }

    pub fn assign(&mut self, name: impl Into<String>, value: Literal) -> bool {
        let name: String = name.into();
        match self.values.get(&name) {
            Some(_) => {
                self.values.insert(name.into(), value);
                true
            }
            None => match self.enclosing {
                None => false,
                Some(env) => env.borrow_mut().assign(name, value),
            },
        }
    }

    pub fn define(&mut self, name: impl Into<String>, value: Literal) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&self, name: impl Into<String>) -> Option<Literal> {
        let name: String = name.into();
        return self.values.get(&name).cloned();
    }

    pub fn fetch(&self, name: impl Into<String>) -> Option<Literal> {
        let name: String = name.into();
        match self.values.get(&name) {
            None => match self.enclosing {
                None => None,
                Some(env) => {
                    let env = env.borrow();
                    let value = env.get(name);
                    return value;
                }
            },
            value => value.cloned(),
        }
    }
}
