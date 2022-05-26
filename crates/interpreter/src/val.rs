use ast::expr::{Expr, Expression};
use meta::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Null,
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            Self::Bool(v) => v.to_string(),
            Self::Int(v) => v.to_string(),
            Self::Float(v) => v.to_string(),
            Self::String(v) => v.clone(),
            Self::Null => String::from("null"),
        };

        f.write_str(format!("{}", msg).as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub val: Val,
    pub span: Span
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
                panic!("ICE: attempted to turn non simple expression {:?} into a Value", expr);
            }
        };

        Self {
            val,
            span: expr.span
        }
    }
}
