use meta::Span;

use crate::{
    expr::Expression,
    op::{Op, Operator},
    stmt::Binding,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Class(String),
    Instance(String),
    Function {
        params: Vec<TypeExpression>,
        returns: Box<TypeExpression>,
    },
    Null,
    Any,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeExpression {
    pub ty: Type,
    pub span: Span,
}

impl TypeExpression {
    pub fn any() -> Self {
        Self {
            ty: Type::Any,
            span: Span::garbage()
        }
    }
}

impl From<Binding> for TypeExpression {
    fn from(binding: Binding) -> Self {
        Self {
            span: binding.name.span,
            ty: binding.ty,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            Self::Int => String::from("int"),
            Self::Float => String::from("float"),
            Self::Bool => String::from("bool"),
            Self::String => String::from("string"),
            Self::Class(name) => name.clone(),
            Self::Instance(name) => format!("instanceof {name}"),
            Self::Function { params, returns } => format!(
                "({}) -> {}",
                params
                    .into_iter()
                    .map(|p| format!("{}", p.ty))
                    .collect::<String>(),
                format!("{}", returns.ty)
            ),
            Self::Null => String::from("null"),
            Self::Any => String::from("any"),
        };

        f.write_str(&msg)
    }
}

pub fn type_compatible(lhs: &Type, rhs: &Type) -> bool {
    match (lhs, rhs) {
        (Type::Int, Type::Float) => true,
        (Type::Float, Type::Int) => true,
        (Type::Null, _) => true,
        (Type::Any, _) => true,
        (_, Type::Any) => true,

        (lhs, rhs) => lhs == rhs,
    }
}

pub fn result_type(lhs: &Expression, op: &Operator, rhs: &Expression) -> Option<Type> {
    match op.op {
        Op::Add => match (&lhs.ty, &rhs.ty) {
            (Type::Int, Type::Int) => Some(Type::Int),
            (Type::Int, Type::Float) => Some(Type::Float),
            (Type::Float, Type::Int) => Some(Type::Float),
            (Type::Float, Type::Float) => Some(Type::Float),
            (Type::String, Type::String) => Some(Type::String),
            (Type::String, Type::Int) => Some(Type::String),
            (Type::Int, Type::String) => Some(Type::String),

            (Type::Null, _) => None,
            (_, Type::Null) => None,

            (Type::Any, _) => Some(Type::Any),
            (_, Type::Any) => Some(Type::Any),

            _ => None,
        },
        Op::Sub => match (&lhs.ty, &rhs.ty) {
            (Type::Int, Type::Int) => Some(Type::Int),
            (Type::Int, Type::Float) => Some(Type::Float),
            (Type::Float, Type::Int) => Some(Type::Float),
            (Type::Float, Type::Float) => Some(Type::Float),

            (Type::Null, _) => None,
            (_, Type::Null) => None,

            (Type::Any, _) => Some(Type::Any),
            (_, Type::Any) => Some(Type::Any),

            _ => None,
        },
        Op::Mul => match (&lhs.ty, &rhs.ty) {
            (Type::Int, Type::Int) => Some(Type::Int),
            (Type::Int, Type::Float) => Some(Type::Float),
            (Type::Float, Type::Int) => Some(Type::Float),
            (Type::Float, Type::Float) => Some(Type::Float),

            (Type::Null, _) => None,
            (_, Type::Null) => None,

            (Type::Any, _) => Some(Type::Any),
            (_, Type::Any) => Some(Type::Any),

            _ => None,
        },
        Op::Div => match (&lhs.ty, &rhs.ty) {
            (Type::Int, Type::Int) => Some(Type::Int),
            (Type::Int, Type::Float) => Some(Type::Float),
            (Type::Float, Type::Int) => Some(Type::Float),
            (Type::Float, Type::Float) => Some(Type::Float),

            (Type::Null, _) => None,
            (_, Type::Null) => None,

            (Type::Any, _) => Some(Type::Any),
            (_, Type::Any) => Some(Type::Any),

            _ => None,
        },
        Op::And | Op::Or => match (&lhs.ty, &rhs.ty) {
            (Type::Bool, Type::Bool) => Some(Type::Bool),

            (Type::Null, _) => None,
            (_, Type::Null) => None,

            (Type::Any, _) => Some(Type::Any),
            (_, Type::Any) => Some(Type::Any),

            _ => None,
        },
        Op::LessThan | Op::LessThanEquals | Op::GreaterThan | Op::GreaterThanEquals => {
            match (&lhs.ty, &rhs.ty) {
                (Type::Int, Type::Int) => Some(Type::Int),
                (Type::Int, Type::Float) => Some(Type::Float),
                (Type::Float, Type::Int) => Some(Type::Float),
                (Type::Float, Type::Float) => Some(Type::Float),

                (Type::Null, _) => None,
                (_, Type::Null) => None,

                (Type::Any, _) => Some(Type::Any),
                (_, Type::Any) => Some(Type::Any),

                _ => None,
            }
        }
        Op::Equals | Op::NotEquals => Some(Type::Bool),
        _ => None,
    }
}
