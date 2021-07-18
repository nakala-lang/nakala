use hir::FunctionDef;

use super::EngineError;
use crate::{func::Function, val::Val};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Env {
    // holds variable definitions
    variables: HashMap<String, Val>,

    // holds function definitions
    functions: HashMap<String, Function>,
}

impl Env {
    pub fn get_variable(&self, variable_name: String) -> Result<Val, EngineError> {
        match self.variables.get(&variable_name) {
            Some(val) => Ok(val.to_owned()),
            None => Err(EngineError::VariableUndefined { variable_name }),
        }
    }

    pub fn set_variable(&mut self, variable_name: String, val: Val) -> Result<Val, EngineError> {
        if self.variables.contains_key(&variable_name) {
            return Err(EngineError::VariableAlreadyExists { variable_name });
        }

        self.variables.insert(variable_name, val);

        Ok(Val::Unit)
    }

    pub fn get_function(&self, function_name: String) -> Result<Function, EngineError> {
        match self.functions.get(&function_name) {
            Some(func) => Ok(func.to_owned()),
            None => Err(EngineError::FunctionUndefined { function_name }),
        }
    }

    pub fn set_function(&mut self, func: FunctionDef) -> Result<Val, EngineError> {
        if self.functions.contains_key(&func.name.to_string()) {
            return Err(EngineError::FunctionAlreadyExists {
                function_name: func.name.to_string(),
            });
        }

        self.functions
            .insert(func.name.to_string(), Function::new(func));

        Ok(Val::Unit)
    }

    pub fn get_all_bindings(&self) -> (Vec<(String, Val)>, Vec<(String, Function)>) {
        (
            self.variables.clone().into_iter().collect(),
            self.functions.clone().into_iter().collect(),
        )
    }
}

impl std::default::Default for Env {
    fn default() -> Self {
        Self {
            variables: HashMap::default(),
            functions: HashMap::default(),
        }
    }
}
