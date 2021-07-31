use hir::{Database, FunctionDef};

use super::EngineError;
use crate::{func::Function, val::Val};
use std::collections::HashMap;

type BindingList = (Vec<(String, Val)>, Vec<(String, Function)>);

#[derive(Clone)]
pub struct Env {
    // holds variable definitions
    variables: HashMap<String, Val>,

    // holds function definitions
    functions: HashMap<String, Function>,
}

impl Env {
    pub fn get_variable(&self, variable_name: &str) -> Result<Val, EngineError> {
        match self.variables.get(variable_name) {
            Some(val) => Ok(val.to_owned()),
            None => Err(EngineError::VariableUndefined {
                variable_name: variable_name.to_string(),
            }),
        }
    }

    pub fn define_variable(&mut self, variable_name: &str, val: Val) -> Result<Val, EngineError> {
        if self.variables.contains_key(variable_name) {
            return Err(EngineError::VariableAlreadyExists {
                variable_name: variable_name.to_string(),
            });
        }

        self.variables.insert(variable_name.to_string(), val);

        Ok(Val::Unit)
    }

    pub fn set_variable(&mut self, variable_name: &str, val: Val) -> Result<Val, EngineError> {
        if !self.variables.contains_key(variable_name) {
            return Err(EngineError::VariableUndefined {
                variable_name: variable_name.to_string(),
            });
        }

        *self.variables.get_mut(variable_name).unwrap() = val;

        Ok(Val::Unit)
    }

    pub fn rename_variable(&mut self, old: &str, new: String) -> Result<(), EngineError> {
        if let Some(old_entry) = self.variables.remove_entry(old) {
            // re-insert with new name
            match self.variables.insert(new.clone(), old_entry.1) {
                Some(_) => Ok(()),
                None => Err(EngineError::VariableAlreadyExists { variable_name: new }),
            }
        } else {
            Err(EngineError::VariableUndefined {
                variable_name: old.to_string(),
            })
        }
    }

    pub fn get_function(&self, function_name: &str) -> Result<Function, EngineError> {
        match self.functions.get(function_name) {
            Some(func) => Ok(func.to_owned()),
            None => Err(EngineError::FunctionUndefined {
                function_name: function_name.to_string(),
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

    pub fn get_all_bindings(&self) -> BindingList {
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
