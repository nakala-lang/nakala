mod database;
pub use database::Database;

use la_arena::Idx;
use smol_str::SmolStr;

pub fn lower(ast: ast::Root) -> (Database, Vec<Stmt>) {
    let mut db = Database::default();
    let stmts = ast.stmts().filter_map(|stmt| db.lower_stmt(stmt)).collect();

    (db, stmts)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    VariableDef(VariableDef),
    Expr(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDef {
    pub name: SmolStr,
    pub value: Expr,
}

pub type ExprIdx = Idx<Expr>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Missing,
    Binary {
        op: BinaryOp,
        lhs: ExprIdx,
        rhs: ExprIdx,
    },
    Literal {
        n: u64,
    },
    Unary {
        op: UnaryOp,
        expr: ExprIdx,
    },
    VariableRef {
        var: SmolStr,
    },
    CodeBlock {
        stmts: Vec<Stmt>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Neg,
}
