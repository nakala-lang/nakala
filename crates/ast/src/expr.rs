use crate::{op::Operator, ty::Type};
use meta::{Span, Spanned};

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
        name: Spanned<String>,
        rhs: Box<Expression>,
    },
    // Logical expressions short circuit, unlike Binary
    Logical {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        paren: Span,
        args: Vec<Expression>,
    },
    Get {
        object: Box<Expression>,
        name: Spanned<String>,
    },
    Set {
        object: Box<Expression>,
        name: Spanned<String>,
        rhs: Box<Expression>,
    },
    IndexGet {
        lhs: Box<Expression>,
        index: Box<Expression>
    },
    IndexSet {
        lhs: Box<Expression>,
        index: Box<Expression>,
        rhs: Box<Expression>
    },
    List(Vec<Expression>),
    This,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub expr: Expr,
    pub span: Span,
    pub ty: Type,
}
