use crate::{op::Operator, ty::Type};
use meta::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Null,
    Unary {
        op: Operator,
        rhs: Box<Expression>,
    },
    Binary {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Variable(String),
    Assign {
        name: String,
        rhs: Box<Expression>
    },
    // Logical expressions short circuit, unlike Binary
    Logical {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub expr: Expr,
    pub span: Span,
    pub ty: Type
}
