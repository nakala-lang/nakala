use crate::{expr::Expression, ty::{Type, TypeExpression}};
use lexer::Token;
use meta::{Span, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    pub name: String,
    pub span: Span,
    pub ty: Type,
}

impl<'a> From<&Token<'a>> for Binding {
    fn from(token: &Token<'a>) -> Self {
        Self {
            name: token.text.to_string(),
            span: token.span,
            ty: Type::Any,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Binding>,
    pub body: Box<Statement>,
    pub return_ty: TypeExpression
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: Spanned<String>,
    pub methods: Vec<Function>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expression),
    Function(Function),
    Class(Class),
    Return(Option<Expression>),
    Print(Expression),
    Variable {
        name: Binding,
        expr: Option<Expression>,
    },
    Block(Vec<Statement>),
    If {
        cond: Expression,
        body: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Until {
        cond: Expression,
        body: Box<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub stmt: Stmt,
    pub span: Span,
}
