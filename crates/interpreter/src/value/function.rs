use ast::{expr::Expression, stmt::Function as AstFunction, ty::type_compatible};
use meta::Span;

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
                args.len(),
            ));
        }

        let new_scope = env.begin_scope(self.closure);

        let params = &self.func.params;
        for (param, arg) in params.iter().zip(args.into_iter()) {
            // TODO try to move this to the parser
            if !type_compatible(&param.ty, &arg.ty) {
                return Err(RuntimeError::IncompatibleTypes(
                    param.name.span.source_id,
                    param.name.span.into(),
                    param.ty.clone(),
                    arg.span.into(),
                    arg.ty,
                ));
            }
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
