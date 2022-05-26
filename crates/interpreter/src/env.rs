use std::collections::HashMap;

use crate::{error::RuntimeError, val::Value};

#[derive(Debug, PartialEq)]
pub struct Env {
    inner: Vec<HashMap<String, Value>>
}

impl Env {
    pub fn new() -> Self { 
        Self {
            inner: vec![HashMap::default()]
        }
    }

    pub fn begin_scope(&mut self) {
        self.inner.push(HashMap::default());
    }

    pub fn end_scope(&mut self) {
        self.inner.pop();
    }

    pub fn get(&self, name: &String) -> Result<Value, RuntimeError> {
        for map in self.inner.iter().rev() {
            if let Some(entry) = map.get(name) {
                return Ok(entry.clone())
            }
        }

        todo!("undefined var");
    }

    pub fn assign(&mut self, name: String, val: Value) -> Result<(), RuntimeError> {
        for map in self.inner.iter_mut().rev() {
            if map.contains_key(&name) {
                map.insert(name, val);
                return Ok(());
            }
        }

        todo!("undefined var");
    }

    pub fn define(&mut self, name: String, val: Value) {
        self.inner.last_mut().expect("env can never not have at least one map").insert(name, val);
    }
}  
