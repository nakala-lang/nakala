mod builtin;
mod class;
mod function;
mod instance;
mod list;
mod val;

use std::{cmp::Ordering, collections::HashMap};

use ast::{
    expr::{Expr, Expression},
    op::Operator,
    stmt::{Statement, Stmt},
    ty::{Type, TypeExpression},
};
pub use builtin::*;
pub use class::*;
pub use function::*;
pub use instance::*;
pub use list::*;
use meta::Span;
pub use val::*;

use crate::{
    env::{Environment, ScopeId},
    error::RuntimeError,
    expr::eval_expr,
};

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        callee_span: Span,
        args: Vec<Expression>,
        env: &mut Environment,
        scope: ScopeId,
    ) -> Result<Value, RuntimeError>;
}

pub trait Indexible {
    fn get(&self, index: usize) -> Result<Value, RuntimeError>;
    fn set(&mut self, index: usize, val: Value) -> Result<(), RuntimeError>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub val: Val,
    pub span: Span,
    pub ty: Type,
}

impl From<Expression> for Value {
    fn from(expr: Expression) -> Self {
        let val = match expr.expr {
            Expr::Bool(v) => Val::Bool(v),
            Expr::Int(v) => Val::Int(v),
            Expr::Float(v) => Val::Float(v),
            Expr::String(v) => Val::String(v),
            Expr::Null => Val::Null,
            _ => {
                panic!(
                    "ICE: attempted to turn non simple expression {:?} into a Value",
                    expr
                );
            }
        };

        Self {
            val,
            span: expr.span,
            ty: expr.ty,
        }
    }
}

impl From<(bool, Span)> for Value {
    fn from(pair: (bool, Span)) -> Self {
        let val = Val::Bool(pair.0);
        let span = pair.1;

        Self {
            val,
            span,
            ty: Type::Bool,
        }
    }
}

impl Value {
    pub fn null() -> Self {
        Self {
            val: Val::Null,
            span: Span::garbage(),
            ty: Type::Null,
        }
    }

    pub fn from_function(stmt: Statement, closure: ScopeId) -> Self {
        if let Stmt::Function(func) = stmt.stmt {
            Value {
                ty: func.ty.ty.clone(),
                val: Val::Function(Function { func, closure }),
                span: stmt.span,
            }
        } else {
            panic!("ICE: from_function should only be called with Stmt::Function")
        }
    }

    pub fn from_class(
        stmt: Statement,
        env: &mut Environment,
        scope: ScopeId,
    ) -> Result<Self, RuntimeError> {
        if let Stmt::Class(class) = stmt.stmt {
            let mut methods: HashMap<String, Value> = HashMap::default();
            let mut statics: HashMap<String, Value> = HashMap::default();

            for method_stmt in class.methods.clone() {
                if let Stmt::Function(func) = &method_stmt.stmt {
                    let name = func.name.item.clone();
                    let runtime_function = Value::from_function(method_stmt, scope);

                    methods.insert(name, runtime_function);
                } else {
                    panic!("ICE: class methods must be Stmt::Function");
                }
            }

            for static_stmt in class.statics.clone() {
                if let Stmt::Variable { name, expr } = static_stmt.stmt {
                    let name = name.name.item.clone();

                    let mut val = Value::null();

                    if let Some(expr) = expr {
                        val = eval_expr(expr, env, scope)?;
                    }

                    statics.insert(name, val);
                } else {
                    panic!("ICE: class statics must be Stmt::Variable");
                }
            }

            Ok(Value {
                ty: Type::Class(class.name.item.clone()),
                val: Val::Class(Class {
                    class,
                    methods,
                    statics,
                }),
                span: stmt.span,
            })
        } else {
            panic!("ICE: from_class should onyl be called with Stmt::Class");
        }
    }

    pub fn add(
        &self,
        env: &mut Environment,
        op: Operator,
        rhs: &Value,
    ) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        match (&self.val, &rhs.val) {
            (Val::Int(lhs), Val::Int(rhs)) => Ok(Value {
                val: Val::Int(lhs + rhs),
                span,
                ty: Type::Int,
            }),
            (Val::String(lhs), Val::String(rhs)) => Ok(Value {
                val: Val::String(format!("{}{}", lhs, rhs)),
                span,
                ty: Type::String,
            }),
            (Val::Int(lhs), Val::String(rhs)) => Ok(Value {
                val: Val::String(format!("{}{}", lhs, rhs)),
                span,
                ty: Type::String,
            }),
            (Val::String(lhs), Val::Int(rhs)) => Ok(Value {
                val: Val::String(format!("{}{}", lhs, rhs)),
                span,
                ty: Type::String,
            }),
            (Val::List { id: lhs_id }, Val::List { id: rhs_id }) => {
                // TODO shouldn't have to clone entire env every time
                let mut cloned_env = env.clone();

                let lhs = env.get_list(*lhs_id);
                let rhs = cloned_env.get_list(*rhs_id);

                lhs.extend_with(rhs.clone());
                Ok(self.clone())
            }
            _ => Err(RuntimeError::UnsupportedOperation(
                self.span.source_id,
                op.span.into(),
                self.span.into(),
                self.ty.clone(),
                rhs.span.into(),
                rhs.ty.clone(),
            )),
        }
    }

    pub fn sub(&self, op: Operator, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        match (&self.val, &rhs.val) {
            (Val::Int(lhs), Val::Int(rhs)) => Ok(Value {
                val: Val::Int(lhs - rhs),
                span,
                ty: Type::Int,
            }),
            (Val::Int(lhs), Val::Float(rhs)) => Ok(Value {
                val: Val::Float(*lhs as f64 - *rhs),
                span,
                ty: Type::Float,
            }),
            (Val::Float(lhs), Val::Int(rhs)) => Ok(Value {
                val: Val::Float(*lhs - *rhs as f64),
                span,
                ty: Type::Float,
            }),
            _ => Err(RuntimeError::UnsupportedOperation(
                self.span.source_id,
                op.span.into(),
                self.span.into(),
                self.ty.clone(),
                rhs.span.into(),
                rhs.ty.clone(),
            )),
        }
    }

    pub fn mul(&self, op: Operator, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        match (&self.val, &rhs.val) {
            (Val::Int(lhs), Val::Int(rhs)) => Ok(Value {
                val: Val::Int(lhs * rhs),
                span,
                ty: Type::Int,
            }),
            (Val::Int(lhs), Val::Float(rhs)) => Ok(Value {
                val: Val::Float(*lhs as f64 * *rhs),
                span,
                ty: Type::Float,
            }),
            (Val::Float(lhs), Val::Int(rhs)) => Ok(Value {
                val: Val::Float(*lhs * *rhs as f64),
                span,
                ty: Type::Float,
            }),
            _ => Err(RuntimeError::UnsupportedOperation(
                self.span.source_id,
                op.span.into(),
                self.span.into(),
                self.ty.clone(),
                rhs.span.into(),
                rhs.ty.clone(),
            )),
        }
    }

    pub fn div(&self, op: Operator, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        match (&self.val, &rhs.val) {
            (Val::Int(lhs), Val::Int(rhs)) => {
                if *rhs != 0 {
                    if lhs % rhs == 0 {
                        Ok(Value {
                            val: Val::Int(lhs / rhs),
                            span,
                            ty: Type::Int,
                        })
                    } else {
                        Ok(Value {
                            val: Val::Float((*lhs as f64) / (*rhs as f64)),
                            span,
                            ty: Type::Float,
                        })
                    }
                } else {
                    todo!("divide by 0")
                }
            }
            _ => Err(RuntimeError::UnsupportedOperation(
                self.span.source_id,
                op.span.into(),
                self.span.into(),
                self.ty.clone(),
                rhs.span.into(),
                rhs.ty.clone(),
            )),
        }
    }

    pub fn eq(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        let val = matches!(self.val.partial_cmp(&rhs.val), Some(Ordering::Equal));

        Ok(Value {
            val: Val::Bool(val),
            span,
            ty: Type::Bool,
        })
    }

    pub fn neq(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        let val = match self.val.partial_cmp(&rhs.val) {
            Some(ordering) => !matches!(ordering, Ordering::Equal),
            None => false,
        };

        Ok(Value {
            val: Val::Bool(val),
            span,
            ty: Type::Bool,
        })
    }

    pub fn lte(&self, op: Operator, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        match (&self.val, &rhs.val) {
            (Val::Int(lhs), Val::Int(rhs)) => Ok((lhs <= rhs, span).into()),
            _ => Err(RuntimeError::UnsupportedOperation(
                self.span.source_id,
                op.span.into(),
                self.span.into(),
                self.ty.clone(),
                rhs.span.into(),
                rhs.ty.clone(),
            )),
        }
    }

    pub fn gt(&self, op: Operator, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        match (&self.val, &rhs.val) {
            (Val::Int(lhs), Val::Int(rhs)) => Ok((lhs > rhs, span).into()),
            _ => Err(RuntimeError::UnsupportedOperation(
                self.span.source_id,
                op.span.into(),
                self.span.into(),
                self.ty.clone(),
                rhs.span.into(),
                rhs.ty.clone(),
            )),
        }
    }

    pub fn gte(&self, op: Operator, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        match (&self.val, &rhs.val) {
            (Val::Int(lhs), Val::Int(rhs)) => Ok((lhs >= rhs, span).into()),
            _ => Err(RuntimeError::UnsupportedOperation(
                self.span.source_id,
                op.span.into(),
                self.span.into(),
                self.ty.clone(),
                rhs.span.into(),
                rhs.ty.clone(),
            )),
        }
    }

    pub fn and(&self, _op: Operator, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        let lhs = self.as_bool()?;
        let rhs = rhs.as_bool()?;

        Ok(Value {
            val: Val::Bool(lhs && rhs),
            span,
            ty: Type::Bool,
        })
    }

    pub fn or(&self, _op: Operator, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        let lhs = self.as_bool()?;
        let rhs = rhs.as_bool()?;

        Ok(Value {
            val: Val::Bool(lhs || rhs),
            span,
            ty: Type::Bool,
        })
    }

    pub fn as_bool(&self) -> Result<bool, RuntimeError> {
        match &self.val {
            Val::Bool(v) => Ok(*v),
            _ => Err(RuntimeError::UnexpectedValueType(
                self.span.source_id,
                Type::Bool,
                format!("{}", self.val),
                self.span.into(),
            )),
        }
    }

    pub fn as_int(&self) -> Result<i64, RuntimeError> {
        match &self.val {
            Val::Int(v) => Ok(*v),
            _ => Err(RuntimeError::UnexpectedValueType(
                self.span.source_id,
                Type::Int,
                format!("{}", self.val),
                self.span.into(),
            )),
        }
    }

    pub fn as_instance(&self) -> Result<InstanceId, RuntimeError> {
        match &self.val {
            Val::Instance { id, .. } => Ok(*id),
            _ => Err(RuntimeError::UnexpectedValueType(
                self.span.source_id,
                Type::Instance(String::from("any")),
                format!("{}", self.val),
                self.span.into(),
            )),
        }
    }

    pub fn as_function(&self) -> Result<Function, RuntimeError> {
        match &self.val {
            Val::Function(func) => Ok(func.clone()),
            _ => Err(RuntimeError::UnexpectedValueType(
                self.span.source_id,
                Type::Function {
                    params: vec![TypeExpression::any()],
                    returns: Box::new(TypeExpression::any()),
                },
                format!("{}", self.val),
                self.span.into(),
            )),
        }
    }

    pub fn as_class(&self) -> Result<Class, RuntimeError> {
        match &self.val {
            Val::Class(class) => Ok(class.clone()),
            _ => Err(RuntimeError::UnexpectedValueType(
                self.span.source_id,
                Type::Class(String::from("unkown")),
                format!("{}", self.val),
                self.span.into(),
            )),
        }
    }

    pub fn bind_this(
        &mut self,
        env: &mut Environment,
        instance: Value,
    ) -> Result<(), RuntimeError> {
        if let Val::Function(ref mut func) = self.val {
            if let Val::Instance { .. } = instance.val {
                let binded_scope = env.begin_scope(func.closure);
                env.define(binded_scope, String::from("this"), instance)?;
                func.closure = binded_scope;
                Ok(())
            } else {
                panic!("Can only bind instance's on functions");
            }
        } else {
            panic!("Can only use 'bind_this' on Val::Function");
        }
    }

    pub fn to_string(&self, env: &mut Environment) -> String {
        self.val.to_string(env)
    }
}
