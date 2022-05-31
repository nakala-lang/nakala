use crate::{error::RuntimeError, val::Value};
use ast::stmt::Class;
use std::collections::HashMap;

pub type InstanceId = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    id: InstanceId,
    class: Class,
    fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(id: InstanceId, class: Class) -> Self {
        Self {
            id,
            class,
            fields: HashMap::default(),
        }
    }

    pub fn get_property(&self, name: &String) -> Result<Value, RuntimeError> {
        if let Some(entry) = self.fields.get(name) {
            return Ok(entry.clone());
        } else {
            todo!("undefined property on instance");
        }
    }

    pub fn set_property(&mut self, name: String, val: Value) -> Result<Value, RuntimeError> {
        self.fields.insert(name, val);

        Ok(Value::null())
    }
}
