use ast::{
    expr::Expression,
    ty::{type_compatible, Type, TypeExpression},
};
use meta::Span;
use parser::{Sym, Symbol};

use crate::{
    env::{Environment, ScopeId},
    error::RuntimeError,
    expr::eval_expr,
};

use super::{Callable, Value};

// Builtins are only functions
pub struct Builtin {
    pub name: String,
    pub ty: Type,
    pub handler: fn(Vec<Value>, &mut Environment) -> Result<Value, RuntimeError>,
}

impl Builtin {
    pub fn new(
        name: String,
        params: Vec<Type>,
        returns: Option<Type>,
        handler: fn(Vec<Value>, &mut Environment) -> Result<Value, RuntimeError>,
    ) -> Self {
        Self {
            name,
            handler,
            ty: Type::Function {
                params: params
                    .clone()
                    .into_iter()
                    .map(|t| TypeExpression {
                        ty: t,
                        span: Span::garbage(),
                    })
                    .collect(),
                returns: Box::new(TypeExpression {
                    span: Span::garbage(),
                    ty: returns.clone().unwrap_or(Type::Null),
                }),
            },
        }
    }

    pub fn as_symbol(&self) -> Symbol {
        if let Type::Function { params, .. } = &self.ty {
            Symbol {
                name: self.name.clone(),
                sym: Sym::Function {
                    arity: params.len(),
                },
                ty: self.ty.clone(),
            }
        } else {
            panic!("ICE: builtin type is not a function signature");
        }
    }
}

impl Clone for Builtin {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            ty: self.ty.clone(),
            handler: self.handler,
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
        if let Type::Function { params, .. } = &self.ty {
            params.len()
        } else {
            panic!("builtin type is not a function signature");
        }
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

        if let Type::Function { params, .. } = &self.ty {
            let mut vals = vec![];
            for (param, arg) in params.iter().zip(args.into_iter()) {
                if !type_compatible(&arg.ty, &param.ty) {
                    todo!("runtime builtin type mismatch");
                }
                vals.push(eval_expr(arg, env, scope)?);
            }

            let mut val = (self.handler)(vals, env)?;
            val.span = Span::combine(&[callee_span, val.span]);
            Ok(val)
        } else {
            panic!("builtin type is not a function signature");
        }
    }
}
