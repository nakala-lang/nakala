use ast::{expr::Expression, op::{Operator, Op}, ty::Type};

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

            (Type::Any, _) => Some(Type::Any),
            (_, Type::Any) => Some(Type::Any),

            _ => None
        }
        Op::Sub => match (&lhs.ty, &rhs.ty) {
            (Type::Int, Type::Int) => Some(Type::Int),
            (Type::Int, Type::Float) => Some(Type::Float),
            (Type::Float, Type::Int) => Some(Type::Float),
            (Type::Float, Type::Float) => Some(Type::Float),

            (Type::Any, _) => Some(Type::Any),
            (_, Type::Any) => Some(Type::Any),
            
            _ => None
        },
        Op::Mul => match (&lhs.ty, &rhs.ty) {
            (Type::Int, Type::Int) => Some(Type::Int),
            (Type::Int, Type::Float) => Some(Type::Float),
            (Type::Float, Type::Int) => Some(Type::Float),
            (Type::Float, Type::Float) => Some(Type::Float),

            (Type::Any, _) => Some(Type::Any),
            (_, Type::Any) => Some(Type::Any),

            _ => None
        },
        Op::Div => match (&lhs.ty, &rhs.ty) {
            (Type::Int, Type::Int) => Some(Type::Int),
            (Type::Int, Type::Float) => Some(Type::Float),
            (Type::Float, Type::Int) => Some(Type::Float),
            (Type::Float, Type::Float) => Some(Type::Float),

            (Type::Any, _) => Some(Type::Any),
            (_, Type::Any) => Some(Type::Any),

            _ => None
        },
        Op::And | Op::Or => match (&lhs.ty, &rhs.ty) {
            (Type::Bool, Type::Bool) => Some(Type::Bool),

            (Type::Any, _) => Some(Type::Any),
            (_, Type::Any) => Some(Type::Any),

            _ => None
        },
        Op::LessThan | Op::LessThanEquals | Op::GreaterThan | Op::GreaterThanEquals => match(&lhs.ty, &rhs.ty) {
            (Type::Int, Type::Int) => Some(Type::Int),
            (Type::Int, Type::Float) => Some(Type::Float),
            (Type::Float, Type::Int) => Some(Type::Float),
            (Type::Float, Type::Float) => Some(Type::Float),

            (Type::Any, _) => Some(Type::Any),
            (_, Type::Any) => Some(Type::Any),

            _ => None
        },
        Op::Equals | Op::NotEquals => Some(Type::Bool),
        _ => None
    }
}
