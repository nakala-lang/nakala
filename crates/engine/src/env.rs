use super::EngineError;
use crate::{class::ClassDef, func::Function, val::Val};
use std::collections::HashMap;

type BindingList = (Vec<(String, Val)>, Vec<(String, Function)>);

#[derive(Clone)]
pub struct Env {
    // holds variable definitions
    variables: HashMap<String, Val>,
    // holds function definitions
    functions: HashMap<String, Function>,
    // holds class definitions
    class_defs: HashMap<String, ClassDef>,

    // FIXME: read comment in `propagate_enclosing_env_changes`
    enclosing_env: Option<Box<Self>>,
}

impl Env {
    pub fn new(enclosing_env: Option<Box<Self>>) -> Self {
        Self {
            variables: HashMap::default(),
            functions: HashMap::default(),
            class_defs: HashMap::default(),
            enclosing_env,
        }
    }

    pub fn get_variable(&self, variable_name: &str) -> Result<Val, EngineError> {
        match self.variables.get(variable_name) {
            Some(val) => Ok(val.to_owned()),
            None => {
                // FIXME probably don't have to clone here
                // should instead listen to the borrow checker
                if let Some(outside_env) = &self.enclosing_env {
                    outside_env.get_variable(variable_name)
                } else {
                    Err(EngineError::VariableUndefined {
                        variable_name: variable_name.to_string(),
                    })
                }
            }
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

    pub fn set_variable(&mut self, variable_name: &str, val: Val) -> Result<(), EngineError> {
        if !self.variables.contains_key(variable_name) {
            if let Some(ref mut outside_env) = self.enclosing_env {
                return outside_env.set_variable(variable_name, val);
            } else {
                return Err(EngineError::VariableUndefined {
                    variable_name: variable_name.to_string(),
                });
            }
        }

        *self.variables.get_mut(variable_name).unwrap() = val;

        Ok(())
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

    pub fn define_class(&mut self, class: ClassDef) -> Result<(), EngineError> {
        if self.class_defs.contains_key(class.name.as_str()) {
            return Err(EngineError::ClassAlreadyDefined { name: class.name });
        }

        self.class_defs.insert(class.name.clone(), class);

        Ok(())
    }

    pub fn get_class_def(&self, class_name: &str) -> Result<ClassDef, EngineError> {
        match self.class_defs.get(class_name) {
            Some(def) => Ok(def.to_owned()),
            None => {
                if let Some(outside_env) = &self.enclosing_env {
                    outside_env.get_class_def(class_name)
                } else {
                    Err(EngineError::ClassUndefined {
                        name: class_name.to_string(),
                    })
                }
            }
        }
    }

    pub fn get_function(&self, function_name: &str) -> Result<Function, EngineError> {
        match self.functions.get(function_name) {
            Some(func) => Ok(func.to_owned()),
            None => {
                if let Some(outside_env) = &self.enclosing_env {
                    outside_env.get_function(function_name)
                } else {
                    Err(EngineError::FunctionUndefined {
                        function_name: function_name.to_string(),
                    })
                }
            }
        }
    }

    pub fn set_function(&mut self, func: Function) -> Result<Val, EngineError> {
        if self.functions.contains_key(&func.name) {
            return Err(EngineError::FunctionAlreadyExists {
                function_name: func.name,
            });
        }

        self.functions.insert(func.name.to_string(), func);

        Ok(Val::Unit)
    }

    pub fn get_all_bindings(&self) -> BindingList {
        (
            self.variables.clone().into_iter().collect(),
            self.functions.clone().into_iter().collect(),
        )
    }

    // FIXME
    //
    // Since we are not passing mutable references of the enclosing_env, and instead are just
    // cloning when needed, we have to propagate the changes to the outside env back to the mutable
    // reference. This is kind of ugly, and we really should change the signature of
    // `enclosing_env` to `Option<Box<&mut Self>>`, but that requires lifetime annotatins which are
    // difficult to get right at the moment
    pub fn propagate_enclosing_env_changes(&mut self, outside_env: &mut Self) {
        if let Some(enclosing_env) = self.enclosing_env.clone() {
            *outside_env = *enclosing_env;
        }
    }
}
