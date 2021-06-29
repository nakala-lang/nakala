use super::EngineError;
use crate::val::Val;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Env {
    // holds variable definitions
    bindings: HashMap<String, Val>,
}

impl Env {
    pub fn get_binding(&self, binding_name: String) -> Result<Val, EngineError> {
        match self.bindings.get(&binding_name) {
            Some(val) => Ok(val.to_owned()),
            None => Err(EngineError::BindingUndefined { binding_name }),
        }
    }

    pub fn set_binding(&mut self, binding_name: String, val: Val) -> Result<Val, EngineError> {
        if self.bindings.contains_key(&binding_name) {
            return Err(EngineError::BindingAlreadyExists { binding_name });
        }

        self.bindings.insert(binding_name, val);

        Ok(Val::Unit)
    }

    pub fn get_all_bindings(&self) -> Vec<(String, Val)> {
        self.bindings.clone().into_iter().collect()
    }
}

impl std::default::Default for Env {
    fn default() -> Self {
        Self {
            bindings: HashMap::default(),
        }
    }
}
