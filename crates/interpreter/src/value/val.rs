use std::cmp::Ordering;


use crate::env::Environment;

use super::{builtin::Builtin, class::Class, Function, InstanceId, ListId};

#[derive(Debug, Clone)]
pub enum Val {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List { id: ListId },
    Function(Function),
    Builtin(Builtin),
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
                Val::List { .. } => Some(Ordering::Less),
                Val::Function(..) => Some(Ordering::Less),
                Val::Builtin(..) => Some(Ordering::Less),
                Val::Class(..) => Some(Ordering::Less),
                Val::Instance { .. } => Some(Ordering::Less),
                Val::Null => Some(Ordering::Less),
            },
            (Val::Int(lhs), rhs) => match rhs {
                Val::Bool(..) => Some(Ordering::Greater),
                Val::Int(rhs) => lhs.partial_cmp(rhs),
                Val::Float(..) => Some(Ordering::Less),
                Val::String(..) => Some(Ordering::Less),
                Val::List { .. } => Some(Ordering::Less),
                Val::Function(..) => Some(Ordering::Less),
                Val::Builtin(..) => Some(Ordering::Less),
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
                Val::List { .. } => Some(Ordering::Less),
                Val::Function(..) => Some(Ordering::Less),
                Val::Builtin(..) => Some(Ordering::Less),
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
                Val::List { .. } => Some(Ordering::Greater),
                Val::Function(..) => Some(Ordering::Greater),
                Val::Builtin(..) => Some(Ordering::Less),
                Val::Class(..) => Some(Ordering::Greater),
                Val::Instance { id: rhs, .. } => lhs.partial_cmp(rhs),
                Val::Null => Some(Ordering::Less),
            },
            (Val::Null, rhs) => match rhs {
                Val::Bool(..) => Some(Ordering::Greater),
                Val::Int(..) => Some(Ordering::Greater),
                Val::Float(..) => Some(Ordering::Greater),
                Val::String(..) => Some(Ordering::Greater),
                Val::List { .. } => Some(Ordering::Greater),
                Val::Function(..) => Some(Ordering::Greater),
                Val::Builtin(..) => Some(Ordering::Less),
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
       let msg = match self {
            Self::Bool(v) => v.to_string(),
            Self::Int(v) => v.to_string(),
            Self::Float(v) => v.to_string(),
            Self::String(v) => v.clone(),
            Self::List { id } => format!("list (id {})", id),
            Self::Null => String::from("null"),
            Self::Function(func) => {
                format!("{} (closure {})", func.func.name.item.clone(), func.closure)
            }
            Self::Builtin(builtin) => format!("{}", builtin),
            Self::Class(class) => class.class.name.item.to_string(),
            Self::Instance { id, name } => format!("{} instance (id {})", name.clone(), id),
        };

       f.write_str(msg.as_str())
    }
}

impl Val {
    pub fn to_string(&self, env: &mut Environment) -> String {
        
        match self {
            Self::Bool(v) => v.to_string(),
            Self::Int(v) => v.to_string(),
            Self::Float(v) => v.to_string(),
            Self::String(v) => v.clone(),
            Self::List { id } => {
                let mut cloned_env = env.clone();
                let list = cloned_env.get_list(*id);
                list.to_string(env)
            },
            Self::Null => String::from("null"),
            Self::Function(func) => {
                format!("{} (closure {})", func.func.name.item.clone(), func.closure)
            }
            Self::Builtin(builtin) => format!("{}", builtin),
            Self::Class(class) => class.class.name.item.to_string(),
            Self::Instance { id, name } => format!("{} instance (id {})", name.clone(), id),
        }
    }
}
