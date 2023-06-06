use crate::literal::Literal;
use std::collections::HashMap;

pub struct Environment {
    frames: Vec<HashMap<String, Literal>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            frames: vec![HashMap::new()],
        }
    }

    pub fn add_frame(&mut self) {
        self.frames.push(HashMap::new());
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    pub fn assign(&mut self, name: impl Into<String>, value: Literal) -> bool {
        let name: String = name.into();
        for frame in self.frames.iter_mut().rev() {
            match frame.get(&name) {
                Some(_) => {
                    frame.insert(name, value);
                    return true;
                }
                None => (),
            }
        }
        return false;
    }

    pub fn define(&mut self, name: impl Into<String>, value: Literal) {
        self.frames
            .iter_mut()
            .last()
            .unwrap()
            .insert(name.into(), value);
    }

    pub fn fetch(&self, name: impl Into<String>) -> Option<Literal> {
        let name: String = name.into();
        for frame in self.frames.iter().rev() {
            match frame.get(&name) {
                Some(value) => return Some(value.clone()),
                None => (),
            }
        }
        return None;
    }
}
