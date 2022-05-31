use std::collections::HashMap;

use ast::{expr::{Expr, Expression}, op::Operator, stmt::{Class, Function}};
use meta::Span;

use crate::{env::ScopeId, error::RuntimeError};

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Null,
    Class(Class),
    Instance {
        class: Class,
        fields: HashMap<String, Value>
    },
    Function {
        func: Function,
        closure: ScopeId,
    }
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            Self::Bool(v) => v.to_string(),
            Self::Int(v) => v.to_string(),
            Self::Float(v) => v.to_string(),
            Self::String(v) => v.clone(),
            Self::Null => String::from("null"),
            Self::Function { func, closure } => format!("{} (closure {})", func.name.item.clone(), closure),
            Self::Class(v) => format!("{}", v.name.item),
            Self::Instance { class, .. } => format!("{} instance", class.name.item)
        };

        f.write_str(format!("{}", msg).as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub val: Val,
    pub span: Span,
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
        }
    }
}

impl Value {
    pub fn null() -> Self {
        Self {
            val: Val::Null,
            span: Span::garbage(),
        }
    }

    pub fn add(&self, op: Operator, rhs: &Value) -> Result<Value, RuntimeError> {
        let span = Span::combine(&[self.span, rhs.span]);

        match (&self.val, &rhs.val) {
            (Val::Int(lhs), Val::Int(rhs)) => {
                Ok(Value {
                    val: Val::Int(lhs + rhs),
                    span
                })
            },
            _ => todo!("unsupported add variant")
        }
    }

    pub fn get_property(&self, name: &String) -> Result<Value, RuntimeError> {
        if let Val::Instance { fields, .. } = &self.val {
            println!("fields: {:#?}", fields.clone());
            if let Some(entry) = fields.get(name) {
                return Ok(entry.clone());
            } else {
                todo!("undefined property on instance");
            }
        }

        todo!("Only instances have properties");
    }

    pub fn set_property(&mut self, name: String, val: Value) -> Result<Value, RuntimeError> {
        if let Val::Instance { ref mut fields, .. } = self.val {
            fields.insert(name, val);

            Ok(Value::null())
        } else {
            todo!("Only instances have properties");
        }
    }
}
