use std::collections::HashMap;

use ast::{expr::Expression, stmt::Class as AstClass};
use meta::Span;

use crate::{
    env::{Environment, ScopeId},
    error::RuntimeError,
};

use super::{Callable, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub class: AstClass,
    pub methods: HashMap<String, Value>,
}

impl Class {
    pub fn constructor(&self) -> Option<Value> {
        match self.methods.get("constructor") {
            Some(func) => Some(func.clone()),
            None => None,
        }
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        self.constructor().map_or(0, |func| {
            func.as_function()
                .expect("constructor has to be a function")
                .arity()
        })
    }

    fn call(
        &self,
        callee_span: Span,
        args: Vec<Expression>,
        env: &mut Environment,
        scope: ScopeId,
    ) -> Result<Value, RuntimeError> {
        if self.arity() != args.len() {
            return Err(RuntimeError::ArityMismatch(
                callee_span.source_id,
                callee_span.into(),
                self.arity(),
                args.len()
            ));
        }

        let val = env.new_instance(self.clone(), Span::garbage());
        let instance = env.get_instance(val.as_instance()?)?;
        if let Ok(mut constructor) = instance.get_property("constructor") {
            // bind this and execute constructor
            constructor.bind_this(env, val.clone())?;
            constructor.as_function()?.call(callee_span, args, env, scope)?;
        }

        Ok(val)
    }
}
