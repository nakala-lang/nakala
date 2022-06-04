use std::cmp::Ordering;

use super::{Function, InstanceId, class::Class};

#[derive(Debug, Clone)]
pub enum Val {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Function(Function),
    Class(Class),
    Instance { id: InstanceId, name: String },
    Null,
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Val::Bool(lhs), rhs) => match rhs {
                Val::Bool(rhs) => lhs.partial_cmp(rhs),
                Val::Int(..) => Some(Ordering::Less),
                Val::Float(..) => Some(Ordering::Less),
                Val::String(..) => Some(Ordering::Less),
                Val::Function(..) => Some(Ordering::Less),
                Val::Class(..) => Some(Ordering::Less),
                Val::Instance { .. } => Some(Ordering::Less),
                Val::Null => Some(Ordering::Less),
            },
            (Val::Int(lhs), rhs) => match rhs {
                Val::Bool(..) => Some(Ordering::Greater),
                Val::Int(rhs) => lhs.partial_cmp(rhs),
                Val::Float(..) => Some(Ordering::Less),
                Val::String(..) => Some(Ordering::Less),
                Val::Function(..) => Some(Ordering::Less),
                Val::Class(..) => Some(Ordering::Less),
                Val::Instance { .. } => Some(Ordering::Less),
                Val::Null => Some(Ordering::Less),
            },
            // float,
            (Val::String(lhs), rhs) => match rhs {
                Val::Bool(..) => Some(Ordering::Greater),
                Val::Int(..) => Some(Ordering::Greater),
                Val::Float(..) => Some(Ordering::Greater),
                Val::String(rhs) => lhs.partial_cmp(rhs),
                Val::Function(..) => Some(Ordering::Less),
                Val::Class(..) => Some(Ordering::Less),
                Val::Instance { .. } => Some(Ordering::Less),
                Val::Null => Some(Ordering::Less),
            },
            // others
            (Val::Instance { id: lhs, .. }, rhs) => match rhs {
                Val::Bool(..) => Some(Ordering::Greater),
                Val::Int(..) => Some(Ordering::Greater),
                Val::Float(..) => Some(Ordering::Greater),
                Val::String(..) => Some(Ordering::Greater),
                Val::Function(..) => Some(Ordering::Greater),
                Val::Class(..) => Some(Ordering::Greater),
                Val::Instance { id: rhs, .. } => lhs.partial_cmp(rhs),
                Val::Null => Some(Ordering::Less),
            },
            (Val::Null, rhs) => match rhs {
                Val::Bool(..) => Some(Ordering::Greater),
                Val::Int(..) => Some(Ordering::Greater),
                Val::Float(..) => Some(Ordering::Greater),
                Val::String(..) => Some(Ordering::Greater),
                Val::Function(..) => Some(Ordering::Greater),
                Val::Class(..) => Some(Ordering::Greater),
                Val::Instance { .. } => Some(Ordering::Greater),
                Val::Null => Some(Ordering::Equal),
            },
            _ => todo!("PartialOrd for {:?} and {:?}", self, other),
        }
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        print!("{:?}", self.partial_cmp(other));
        self.partial_cmp(other).map_or(false, Ordering::is_eq)
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
            Self::Function(func) => {
                format!("{} (closure {})", func.func.name.item.clone(), func.closure)
            }
            Self::Class(class) => class.class.name.item.to_string(),
            Self::Instance { id, name } => format!("{} instance (id {})", name.clone(), id),
        };

        f.write_str(&msg)
    }
}
