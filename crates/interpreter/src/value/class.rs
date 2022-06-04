use std::collections::HashMap;

use ast::{expr::Expression, stmt::Class as AstClass};
use meta::Span;

use crate::{env::{Environment, ScopeId}, error::RuntimeError};

use super::{Callable, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub class: AstClass,
    pub methods: HashMap<String, Value>,
}

impl Callable for Class {
    fn call(&self, args: Vec<Expression>, env: &mut Environment, scope: ScopeId) -> Result<Value, RuntimeError> {
        let val = env.new_instance(self.clone(), Span::garbage());
        let instance = env.get_instance(val.as_instance()?)?;
        if let Ok(mut constructor) = instance.get_property("constructor") {
            // bind this and execute constructor
            constructor.bind_this(env, val.clone())?;
            constructor.as_function()?.call(args, env, scope)?;
        }

        Ok(val)
    }
}
