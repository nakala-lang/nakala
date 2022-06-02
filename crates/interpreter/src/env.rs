use ast::{stmt::Class, ty::Type};
use meta::{trace, Span};

use crate::{error::RuntimeError, instance::{Instance, InstanceId}, val::{self, Val, Value}};
use std::{collections::HashMap, fmt::Debug};

pub type ScopeId = usize;

#[derive(Clone, PartialEq)]
pub struct Environment {
    scopes: Vec<Scope>,
    next_scope_id: ScopeId,
    instances: Vec<Instance>,
    next_instance_id: InstanceId,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(0, None)],
            next_scope_id: 1,
            instances: vec![],
            next_instance_id: 0,
        }
    }

    pub fn new_instance(&mut self, class: val::Class, span: Span) -> Value {
        let id = self.next_instance_id;
        self.next_instance_id = self.next_instance_id + 1;

        let name = class.class.name.item.clone();

        self.instances.push(Instance::new(id, class));

        Value {
            ty: Type::Instance(name.clone()),
            val: Val::Instance { id, name },
            span,
        }
    }

    pub fn get_instance(&mut self, instance_id: InstanceId) -> Result<&mut Instance, RuntimeError> {
        Ok(self
            .instances
            .get_mut(instance_id)
            .expect("ICE: Called get instance on instance that doesn't exist"))
    }

    pub fn begin_scope(&mut self, closure: Option<ScopeId>) -> ScopeId {
        let id = self.next_scope_id;
        self.next_scope_id = self.next_scope_id + 1;

        let enclosing_id = closure.unwrap_or_else(|| {
            id.checked_sub(1)
                .expect("ICE: called begin scope without on the root scope")
        });

        self.scopes.push(Scope::new(id, Some(enclosing_id)));

        trace!(format!("created scope {:?}", self.scopes.last().unwrap()));

        id
    }

    pub fn get(&self, scope_id: ScopeId, name: &String) -> Result<Value, RuntimeError> {
        trace!(format!("trying to get {} in scope {}", name, scope_id));
        let env = self
            .scopes
            .get(scope_id)
            .expect("ICE: no matching Scope for ScopeId");

        match env.get(name) {
            Ok(v) => Ok(v),
            Err(e) => {
                trace!(format!(
                    "didnt find - checking enclosing scope {:?}",
                    env.enclosing
                ));
                if let Some(enclosing_id) = env.enclosing {
                    self.get(enclosing_id, name)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn assign(
        &mut self,
        scope_id: ScopeId,
        name: String,
        val: Value,
    ) -> Result<(), RuntimeError> {
        let env = self
            .scopes
            .get_mut(scope_id)
            .expect("ICE: no matching Scope for ScopeId");

        match env.assign(name.clone(), val.clone()) {
            Err(e) => {
                if let Some(enclosing_id) = env.enclosing {
                    self.assign(enclosing_id, name, val)
                } else {
                    Err(e)
                }
            }
            _ => Ok(()),
        }
    }

    pub fn define(
        &mut self,
        scope_id: ScopeId,
        name: String,
        val: Value,
    ) -> Result<(), RuntimeError> {
        let env = self
            .scopes
            .get_mut(scope_id)
            .expect("ICE: no matching Scope for ScopeId");

        match env.define(name.clone(), val.clone()) {
            Err(e) => {
                if let Some(enclosing_id) = env.enclosing {
                    self.define(enclosing_id, name, val)
                } else {
                    Err(e)
                }
            }
            _ => Ok(()),
        }
    }
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Environment has {} Scopes", self.scopes.len())?;

        for scope in self.scopes.iter() {
            writeln!(f, "Scope {}", scope.id)?;
            writeln!(f, "\t- Enclosing: {:?}", scope.enclosing)?;
            writeln!(f, "\t- Values:")?;
            for (key, value) in scope.values.iter() {
                writeln!(f, "\t\t- {} = {}", key, value)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Scope {
    pub id: ScopeId,
    pub values: HashMap<String, Value>,
    pub enclosing: Option<ScopeId>,
}

impl Scope {
    pub fn new(id: ScopeId, enclosing: Option<ScopeId>) -> Self {
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

        Err(RuntimeError::UndefinedVariable)
    }

    pub fn assign(&mut self, name: String, val: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name) {
            self.values.insert(name, val);
            return Ok(());
        }

        Err(RuntimeError::UndefinedVariable)
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
