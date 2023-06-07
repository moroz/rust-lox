use crate::literal::Literal;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn enclose(enclosing: &Rc<RefCell<Self>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Rc::clone(enclosing)),
        }
    }

    pub fn assign(&mut self, name: impl Into<String>, value: Literal) -> bool {
        let name: String = name.into();
        match self.values.get(&name) {
            Some(_) => {
                self.values.insert(name, value);
                return true;
            }
            None => match self.enclosing.clone() {
                Some(enclosing) => return enclosing.borrow_mut().assign(&name, value),
                None => {
                    return false;
                }
            },
        }
    }

    pub fn define(&mut self, name: impl Into<String>, value: Literal) {
        self.values.insert(name.into(), value);
    }

    pub fn fetch(&self, name: impl Into<String>) -> Option<Literal> {
        let name: String = name.into();
        match self.values.get(&name) {
            Some(value) => {
                return Some(value.clone());
            }
            None => match self.enclosing.clone() {
                Some(enclosing) => {
                    return enclosing.borrow_mut().fetch(&name);
                }
                None => {
                    return None;
                }
            },
        }
    }
}
