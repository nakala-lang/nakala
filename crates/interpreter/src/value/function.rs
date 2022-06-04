use ast::{expr::Expression, stmt::Function as AstFunction};

use crate::{
    env::{Environment, ScopeId},
    error::RuntimeError,
    eval_block,
    expr::eval_expr,
};

use super::{Callable, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub func: AstFunction,
    pub closure: ScopeId,
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.func.params.len()
    }

    fn call(
        &self,
        args: Vec<Expression>,
        env: &mut Environment,
        scope: ScopeId,
    ) -> Result<Value, RuntimeError> {
        if self.arity() != args.len() {
            todo!("parity mismatch");
        }

        let new_scope = env.begin_scope(self.closure);

        let params = &self.func.params;
        for (param, arg) in params.into_iter().zip(args.into_iter()) {
            let val = eval_expr(arg, env, scope)?;
            env.define(new_scope, param.name.item.clone(), val)?;
        }

        match eval_block(*self.func.body.clone(), env, new_scope) {
            Ok(()) => Ok(Value::null()),
            Err(RuntimeError::EarlyReturn(val)) => Ok(val),

            Err(other) => Err(other),
        }
    }
}
