use crate::{error::RuntimeError, val::Value};
use std::collections::HashMap;

pub type EnvId = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    inner: Vec<Env>,
    next_id: usize
}

impl Environment {
    pub fn new() -> Self {
        Self {
            inner: vec![Env::new(0, None)],
            next_id: 1
        }
    }

    pub fn begin_scope(&mut self, closure: Option<EnvId>) -> EnvId {
        let id = self.next_id;
        let enclosing_id = closure.unwrap_or_else(|| {
            self.next_id = self.next_id + 1;
            id.checked_sub(1).expect("ICE: called begin scope without on the root scope")
        });

        self.inner.push(Env::new(id, Some(enclosing_id)));

        id
    }

    pub fn get(&self, env: EnvId, name: &String) -> Result<Value, RuntimeError> {
        let env = self.inner.get(env).expect("ICE: no matching Env for EnvId");
        
        match env.get(name) {
            Ok(v) => Ok(v),
            Err(e) => {
                if let Some(enclosing_env_id) = env.enclosing {
                    self.get(enclosing_env_id, name)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn assign(&mut self, env: EnvId, name: String, val: Value) -> Result<(), RuntimeError> {
        let env = self.inner.get_mut(env).expect("ICE: no matching Env for EnvId");

        match env.assign(name.clone(), val.clone()) {
            Err(e) => {
                if let Some(enclosing_env_id) = env.enclosing {
                    self.assign(enclosing_env_id, name, val)
                } else {
                    Err(e)
                }
            },
            _ => Ok(())
        }
    }

    pub fn define(&mut self, env: EnvId, name: String, val: Value) -> Result<(), RuntimeError> {
        let env = self.inner.get_mut(env).expect("ICE: no matching Env for EnvId");

        match env.define(name.clone(), val.clone()) {
            Err(e) => {
                if let Some(enclosing_env_id) = env.enclosing {
                    self.define(enclosing_env_id, name, val)
                } else {
                    Err(e)
                }
            },
            _ => Ok(())
        }
    }

    pub fn debug(&self) {
        println!("Total of {} envs", self.inner.len());
        for env in self.inner.iter() {
            println!("env {}", env.id);
            for (key, value) in env.values.iter() {
                println!("{} = {}", key, value);
            }
            println!("-----------");
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Env {
    pub id: EnvId,
    pub values: HashMap<String, Value>,
    pub enclosing: Option<EnvId>,
}

impl Env {
    pub fn new(id: EnvId, enclosing: Option<EnvId>) -> Self {
        Self {
            id,
            values: HashMap::default(),
            enclosing,
        }
    }

    pub fn get(&self, name: &String) -> Result<Value, RuntimeError> {
        if let Some(entry) = self.values.get(name) {
            return Ok(entry.clone());
        }

        todo!("undefined var");
    }

    pub fn assign(&mut self, name: String, val: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name) {
            self.values.insert(name, val);
            return Ok(());
        }

        todo!("undefined var");
    }

    pub fn define(&mut self, name: String, val: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name) {
            todo!("can't define var that already exists");
        } else {
            self.values.insert(name, val);
            Ok(())
        }
    }
}
