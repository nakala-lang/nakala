use crate::{
    error::RuntimeError,
    val::{Class, Value},
};
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
        // add all the methods to the fields
        let mut fields = HashMap::default();
        for (key, value) in class.methods.clone() {
            fields.insert(key, value);
        }

        Self { id, class, fields }
    }

    pub fn get_property(&self, name: &str) -> Result<Value, RuntimeError> {
        if let Some(entry) = self.fields.get(name) {
            Ok(entry.clone())
        } else {
            Err(RuntimeError::UndefinedClassProperty(
                self.class.class.name.span.source_id,
                self.class.class.name.span.into(),
                name.to_string(),
            ))
        }
    }

    pub fn set_property(&mut self, name: String, val: Value) -> Result<Value, RuntimeError> {
        self.fields.insert(name, val);

        Ok(Value::null())
    }
}
