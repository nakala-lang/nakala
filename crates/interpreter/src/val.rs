use std::collections::HashMap;

use ast::{expr::{Expr, Expression}, op::Operator, stmt::{Class as AstClass, Function as AstFunction, Statement, Stmt}, ty::Type};
use meta::Span;

use crate::{env::ScopeId, error::RuntimeError, instance::InstanceId};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub func: AstFunction,
    pub closure: ScopeId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub class: AstClass,
    pub methods: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Null,
    Function(Function),
    Class(Class),
    Instance { id: InstanceId, name: String },
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            Self::Bool(v) => v.to_string(),
            Self::Int(v) => v.to_string(),
            Self::Float(v) => v.to_string(),
            Self::String(v) => v.clone(),
            Self::Null => String::from("null"),
            Self::Function(func) => {
                format!("{} (closure {})", func.func.name.item.clone(), func.closure)
            }
            Self::Class(class) => format!("{}", class.class.name.item),
            Self::Instance { id, name } => format!("{} instance (id {})", name.clone(), id),
        };

        f.write_str(format!("{}", msg).as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub val: Val,
    pub span: Span,
    pub ty: Type
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.val).as_str())
    }
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
            ty: expr.ty
        }
    }
}

impl Value {
    pub fn null() -> Self {
        Self {
            val: Val::Null,
            span: Span::garbage(),
            ty: Type::Null
        }
    }

    pub fn from_function(stmt: Statement, closure: ScopeId) -> Self {
        if let Stmt::Function(func) = stmt.stmt {
            Value {
                val: Val::Function(Function { func, closure }),
                span: stmt.span,
                ty: Type::Any // TODO: function types
            }
        } else {
            panic!("ICE: from_function should only be called with Stmt::Function")
        }
    }

    pub fn from_class(stmt: Statement, scope: ScopeId) -> Self {
        if let Stmt::Class(class) = stmt.stmt {
            let mut methods: HashMap<String, Value> = HashMap::default();
            for method_stmt in class.methods.clone() {
                if let Stmt::Function(func) = &method_stmt.stmt {
                    let name = func.name.item.clone();
                    let runtime_function = Value::from_function(method_stmt, scope);

                    methods.insert(name, runtime_function);
                } else {
                    panic!("ICE: class methods must be Stmt::Function");
                }
            }

            Value {
                ty: Type::Class(class.name.item.clone()),
                val: Val::Class(Class { class, methods }),
                span: stmt.span,
            }
        } else {
            panic!("ICE: from_class should onyl be called with Stmt::Class");
        }
    }

    pub fn add(&self, op: Operator, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        match (&self.val, &rhs.val) {
            (Val::Int(lhs), Val::Int(rhs)) => Ok(Value {
                val: Val::Int(lhs + rhs),
                span,
                ty: Type::Int
            }),
            _ => todo!("unsupported add variant"),
        }
    }

    pub fn is_truthy(&self) -> bool {
        true
    }

    pub fn as_instance(&self) -> Result<InstanceId, RuntimeError> {
        match &self.val {
            Val::Instance { id, .. } => Ok(*id),
            _ => Err(RuntimeError::ExpectedInstance(
                self.span.source_id,
                self.span.into(),
            )),
        }
    }
}
