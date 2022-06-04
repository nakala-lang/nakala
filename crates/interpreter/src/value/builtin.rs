use ast::{expr::Expression, ty::{Type, type_compatible}};

use crate::{env::{Environment, ScopeId}, error::RuntimeError, expr::eval_expr};

use super::{Callable, Value};

// Builtins are only functions
pub struct Builtin {
    pub name: String,
    pub params: Vec<Type>,
    pub handler: fn(Vec<Value>) -> Value
}

impl Clone for Builtin {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            params: self.params.clone(),
            handler: self.handler
        }
    }
}

impl std::fmt::Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin> {}", self.name)
    }
}

impl std::fmt::Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin> {}", self.name)
    }
}

impl Callable for Builtin {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, args: Vec<Expression>, env: &mut Environment, scope: ScopeId) -> Result<Value, RuntimeError> {
        if self.arity() != args.len() {
            todo!("builtin parity mismatch");
        }

        let mut vals = vec![];
        let params = &self.params;
        for (param, arg) in params.into_iter().zip(args.into_iter()) {
            if !type_compatible(&arg.ty, param) {
                todo!("runtime builtin type mismatch");
            }
            vals.push(eval_expr(arg, env, scope)?);
        }

        Ok((self.handler)(vals))
    }
}
