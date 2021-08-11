mod database;
pub use database::Database;

use la_arena::Idx;
use smol_str::SmolStr;

#[derive(Clone)]
pub struct Hir {
    pub db: Database,
    pub stmts: Vec<Stmt>,
}

pub fn lower(ast: ast::Root) -> Hir {
    let mut db = Database::default();
    let stmts = ast.stmts().filter_map(|stmt| db.lower_stmt(stmt)).collect();

    Hir { db, stmts }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    VariableDef(VariableDef),
    VariableAssign(VariableAssign),
    Expr(Expr),
    FunctionDef(FunctionDef),
    If(If),
    ElseIf(ElseIf),
    Else(Else),
    Return(Return),
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDef {
    pub name: SmolStr,
    pub value: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableAssign {
    pub name: SmolStr,
    pub value: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct If {
    pub expr: Expr,
    pub body: CodeBlock,
    pub else_branch: Option<Box<ElseBranch>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElseIf {
    pub if_stmt: If,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Else {
    pub body: CodeBlock,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ElseBranch {
    Else(Else),
    ElseIf(ElseIf),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDef {
    pub name: SmolStr,
    pub param_ident_list: Vec<SmolStr>,
    pub body: CodeBlock,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub value: Option<Expr>,
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
    Number {
        n: f64,
    },
    String {
        s: String,
    },
    Boolean {
        b: bool,
    },
    Unary {
        op: UnaryOp,
        expr: ExprIdx,
    },
    VariableRef {
        var: SmolStr,
    },
    CodeBlock(CodeBlock),
    FunctionCall {
        name: String,
        param_value_list: Vec<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct CodeBlock {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    ComparisonEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}
