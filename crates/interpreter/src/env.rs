use ast::{stmt::Function, ty::Type};
use meta::{trace, Span, Spanned};

use crate::{error::RuntimeError, value::{self, Instance, InstanceId, Val, Value, Builtin}};
use std::collections::{hash_map::Entry, HashMap};

pub type ScopeId = usize;

#[derive(Clone, PartialEq, Default)]
pub struct Environment {
    pub scopes: Vec<Scope>,
    next_scope_id: ScopeId,
    instances: Vec<Instance>,
    next_instance_id: InstanceId,
}

impl Environment {
    pub fn new(builtins: Vec<Builtin>) -> Result<Self, RuntimeError> {
        let mut env = Self {
            scopes: vec![Scope::new(0, None)],
            next_scope_id: 1,
            instances: vec![],
            next_instance_id: 0,
        };

        env.define_builtins(builtins)?;

        Ok(env)
    }

    fn define_builtins(&mut self, builtins: Vec<Builtin>) -> Result<(), RuntimeError> {
        for builtin in builtins {
            self.define(0, builtin.name.clone(), Value {
                val: Val::Builtin(builtin),
                span: Span::garbage(),
                ty: Type::Any
            })?;
        }

        Ok(())
    }

    pub fn new_instance(&mut self, class: value::Class, span: Span) -> Value {
        let id = self.next_instance_id;
        self.next_instance_id += 1;

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

    pub fn begin_scope(&mut self, enclosing: ScopeId) -> ScopeId {
        let id = self.next_scope_id;
        self.next_scope_id += 1;

        self.scopes.push(Scope::new(id, Some(enclosing)));

        trace!(format!("created scope {:?}", self.scopes.last().unwrap()));

        id
    }

    pub fn delete_scope(&mut self, _scope_id: ScopeId) {
        // TODO - handle cleaning up non closure scopes that we don't need to stick around
        // for example, after evaluating an if statement block

        //todo!("delete scope")
    }

    pub fn get(&self, scope_id: ScopeId, name: &Spanned<String>) -> Result<Value, RuntimeError> {
        let scope = self
            .scopes
            .get(scope_id)
            .expect("ICE: no matching Scope for ScopeId");
        trace!(format!(
            "trying to get {} in scope {:#?}",
            name.item, &scope
        ));

        match scope.get(name) {
            Ok(v) => Ok(v),
            Err(e) => {
                trace!(format!(
                    "didnt find - checking enclosing scope {:?}",
                    scope.enclosing
                ));
                if let Some(enclosing_id) = scope.enclosing {
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
        name: Spanned<String>,
        val: Value,
    ) -> Result<(), RuntimeError> {
        let scope = self
            .scopes
            .get_mut(scope_id)
            .expect("ICE: no matching Scope for ScopeId");

        match scope.assign(name.clone(), val.clone()) {
            Err(e) => {
                if let Some(enclosing_id) = scope.enclosing {
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
        let scope = self
            .scopes
            .get_mut(scope_id)
            .expect("ICE: no matching Scope for ScopeId");

        match scope.define(name.clone(), val.clone()) {
            Err(e) => {
                if let Some(enclosing_id) = scope.enclosing {
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
            writeln!(f, "{:?}", scope)?;
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct Scope {
    pub id: ScopeId,
    values: HashMap<String, Value>,
    pub enclosing: Option<ScopeId>,
}

impl std::fmt::Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Scope {}", self.id)?;
        writeln!(f, "\t- Enclosing: {:?}", self.enclosing)?;
        writeln!(f, "\t- Values:")?;
        for (key, value) in self.values.iter() {
            writeln!(f, "\t\t- {} = {}", key, value.ty)?;
        }

        Ok(())
    }
}

impl Scope {
    pub fn new(id: ScopeId, enclosing: Option<ScopeId>) -> Self {
        Self {
            id,
            values: HashMap::default(),
            enclosing,
        }
    }

    pub fn get(&self, name: &Spanned<String>) -> Result<Value, RuntimeError> {
        if let Some(entry) = self.values.get(&name.item) {
            return Ok(entry.clone());
        }

        Err(RuntimeError::UndefinedVariable(
            name.span.source_id,
            name.span.into(),
        ))
    }

    pub fn assign(&mut self, name: Spanned<String>, val: Value) -> Result<(), RuntimeError> {
        if let Entry::Occupied(mut e) = self.values.entry(name.item) {
            e.insert(val);
            Ok(())
        } else {
            Err(RuntimeError::UndefinedVariable(
                name.span.source_id,
                name.span.into(),
            ))
        }
    }

    pub fn define(&mut self, name: String, val: Value) -> Result<(), RuntimeError> {
        if let Entry::Vacant(e) = self.values.entry(name) {
            e.insert(val);
            Ok(())
        } else {
            todo!("can't define var that already exists");
        }
    }
}
