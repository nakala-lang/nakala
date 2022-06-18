use crate::{
    expr::Expression,
    ty::{Type, TypeExpression},
};
use lexer::Token;
use meta::{Span, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    pub name: Spanned<String>,
    pub ty: Type,
}

impl From<&Token> for Binding {
    fn from(token: &Token) -> Self {
        Self {
            name: Spanned {
                item: token.text.to_string(),
                span: token.span,
            },
            ty: Type::Any,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Spanned<String>,
    pub params: Vec<Binding>,
    pub body: Box<Statement>,
    pub ty: TypeExpression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: Spanned<String>,
    pub methods: Vec<Statement>,
    pub statics: Vec<Statement>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expression),
    Function(Function),
    Class(Class),
    Return(Option<Expression>),
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
