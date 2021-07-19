use hir::{Database, FunctionDef};

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
    pub fn get_variable(&self, variable_name: &String) -> Result<Val, EngineError> {
        match self.variables.get(variable_name) {
            Some(val) => Ok(val.to_owned()),
            None => Err(EngineError::VariableUndefined {
                variable_name: variable_name.clone(),
            }),
        }
    }

    pub fn set_variable(&mut self, variable_name: &String, val: Val) -> Result<Val, EngineError> {
        if self.variables.contains_key(variable_name) {
            return Err(EngineError::VariableAlreadyExists {
                variable_name: variable_name.clone(),
            });
        }

        self.variables.insert(variable_name.clone(), val);

        Ok(Val::Unit)
    }

    pub fn rename_variable(&mut self, old: &String, new: String) -> Result<(), EngineError> {
        if let Some(old_entry) = self.variables.remove_entry(old) {
            // re-insert with new name
            match self.variables.insert(new.clone(), old_entry.1) {
                Some(_) => Ok(()),
                None => Err(EngineError::VariableAlreadyExists { variable_name: new }),
            }
        } else {
            Err(EngineError::VariableUndefined {
                variable_name: old.clone(),
            })
        }
    }

    pub fn get_function(&self, function_name: &String) -> Result<Function, EngineError> {
        match self.functions.get(function_name) {
            Some(func) => Ok(func.to_owned()),
            None => Err(EngineError::FunctionUndefined {
                function_name: function_name.clone(),
            }),
        }
    }

    pub fn set_function(
        &mut self,
        func: FunctionDef,
        funcs_db: Database,
    ) -> Result<Val, EngineError> {
        if self.functions.contains_key(&func.name.to_string()) {
            return Err(EngineError::FunctionAlreadyExists {
                function_name: func.name.to_string(),
            });
        }

        self.functions
            .insert(func.name.to_string(), Function::new(func, funcs_db));

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
