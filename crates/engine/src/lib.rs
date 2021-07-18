use hir::{
    BinaryOp, CodeBlock, Database, Expr, ExprIdx, FunctionDef, Hir, Stmt, UnaryOp, VariableDef,
};
use std::ops::Index;

pub mod env;
pub mod error;
pub mod func;
pub mod val;

use env::Env;
use error::EngineError;
use val::Val;

pub fn eval(env: &mut Env, hir: Hir) -> Result<Val, EngineError> {
    let db = hir.db;
    let stmts = hir.stmts;
    let mut return_val = Val::Unit;

    for stmt in stmts {
        return_val = eval_stmt(env, &db, stmt)?;
    }

    Ok(return_val)
}

fn eval_stmt(env: &mut Env, db: &Database, stmt: Stmt) -> Result<Val, EngineError> {
    match stmt {
        Stmt::Expr(expr) => eval_expr(env, &db, expr),
        Stmt::VariableDef(VariableDef { name, value }) => {
            eval_variable_def(env, &db, name.to_string(), value)
        }
        Stmt::FunctionDef(func_def) => eval_function_def(env, func_def),
    }
}

fn eval_code_block(env: &Env, db: &Database, stmts: Vec<Stmt>) -> Result<Val, EngineError> {
    let mut block_env = env.clone();
    let mut return_val = Val::Unit;

    for stmt in stmts {
        return_val = eval_stmt(&mut block_env, &db, stmt)?;
    }

    Ok(return_val)
}

fn eval_function_def(env: &mut Env, func_def: FunctionDef) -> Result<Val, EngineError> {
    env.set_function(func_def)
}

fn eval_expr(env: &Env, db: &Database, expr: Expr) -> Result<Val, EngineError> {
    match expr {
        Expr::Binary { op, lhs, rhs } => eval_binary_expr(&env, &db, op, lhs, rhs),
        Expr::Number { n } => Ok(Val::Number(n.into())),
        Expr::String { s } => Ok(Val::String(s)),
        Expr::VariableRef { var } => env.get_variable(var.to_string()),
        Expr::Unary { op, expr } => eval_unary_expr(env, &db, op, db.exprs.index(expr).to_owned()),
        Expr::CodeBlock(CodeBlock { stmts }) => eval_code_block(env, &db, stmts),
        _ => Err(EngineError::InvalidExpression(expr)),
    }
}

fn eval_variable_def(
    env: &mut Env,
    db: &Database,
    name: String,
    value: Expr,
) -> Result<Val, EngineError> {
    let val = eval_expr(env, db, value)?;
    env.set_variable(name, val)
}

fn eval_unary_expr(env: &Env, db: &Database, op: UnaryOp, expr: Expr) -> Result<Val, EngineError> {
    let val = eval_expr(env, db, expr)?;
    match op {
        UnaryOp::Neg => val.neg(),
    }
}

fn eval_binary_expr(
    env: &Env,
    db: &Database,
    op: BinaryOp,
    lhs: ExprIdx,
    rhs: ExprIdx,
) -> Result<Val, EngineError> {
    let lhs_val = eval_expr(&env, &db, db.exprs.index(lhs).to_owned())?;
    let rhs_val = eval_expr(&env, &db, db.exprs.index(rhs).to_owned())?;
    match op {
        BinaryOp::Add => lhs_val.add(rhs_val),
        BinaryOp::Sub => lhs_val.sub(rhs_val),
        BinaryOp::Mul => lhs_val.mul(rhs_val),
        BinaryOp::Div => lhs_val.div(rhs_val),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn eval_add() {}
}
