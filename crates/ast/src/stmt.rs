use crate::{expr::{Expr, Expression}, ty::Type};
use meta::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expression),
    Print(Expression),
    Variable { name: String, expr: Option<Expression> },
    Block(Vec<Statement>),
    If {
        cond: Expression,
        body: Box<Statement>,
        else_branch: Option<Box<Statement>>
    },
    Until {
        cond: Expression,
        body: Box<Statement>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub stmt: Stmt,
    pub span: Span,
}
