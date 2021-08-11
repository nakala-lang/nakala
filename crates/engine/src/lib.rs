use hir::{
    BinaryOp, CodeBlock, Database, ElseBranch, Expr, ExprIdx, FunctionDef, Hir, If, Stmt, UnaryOp,
    VariableAssign, VariableDef,
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
        Stmt::Expr(expr) => eval_expr(env, db, expr),
        Stmt::VariableDef(VariableDef { name, value }) => {
            eval_variable_def(env, db, name.to_string(), value)
        }
        Stmt::VariableAssign(VariableAssign { name, value }) => {
            eval_variable_assign(env, db, name.to_string(), value)
        }
        Stmt::FunctionDef(func_def) => eval_function_def(env, db, func_def),
        Stmt::If(if_stmt) => eval_if_stmt(env, db, if_stmt),
        Stmt::ElseIf(else_if) => eval_if_stmt(env, db, else_if.if_stmt),
        Stmt::Else(else_stmt) => eval_code_block(env, db, else_stmt.body.stmts),
        Stmt::Return(return_stmt) => eval_expr(env, db, return_stmt.value),
    }
}

fn eval_code_block(env: &mut Env, db: &Database, stmts: Vec<Stmt>) -> Result<Val, EngineError> {
    let mut block_env = Env::new(Some(Box::new(env.clone())));
    let mut return_val = Val::Unit;

    for stmt in stmts {
        if let Stmt::Return(r) = stmt.clone() {
            block_env.propagate_enclosing_env_changes(env);
            return Err(EngineError::EarlyReturn {
                value: eval_expr(&mut block_env, db, r.value)?,
            });
        } else {
            return_val = eval_stmt(&mut block_env, db, stmt)?;
        }
    }

    block_env.propagate_enclosing_env_changes(env);

    Ok(return_val)
}

fn eval_function_def(
    env: &mut Env,
    db: &Database,
    func_def: FunctionDef,
) -> Result<Val, EngineError> {
    env.set_function(func_def, db.clone())
}

fn eval_expr(env: &mut Env, db: &Database, expr: Expr) -> Result<Val, EngineError> {
    match expr {
        Expr::Binary { op, lhs, rhs } => eval_binary_expr(env, db, op, lhs, rhs),
        Expr::Number { n } => Ok(Val::Number(n)),
        Expr::String { s } => Ok(Val::String(s)),
        Expr::Boolean { b } => Ok(Val::Boolean(b)),
        Expr::VariableRef { var } => env.get_variable(&var.to_string()),
        Expr::Unary { op, expr } => eval_unary_expr(env, db, op, db.exprs.index(expr).to_owned()),
        Expr::CodeBlock(CodeBlock { stmts }) => eval_code_block(env, db, stmts),
        Expr::FunctionCall {
            name,
            param_value_list,
        } => eval_function_call(env, db, name, param_value_list),
        Expr::Missing => {
            unreachable!("Missing tokens will get caught before they reach the engine")
        }
    }
}

fn eval_function_call(
    env: &mut Env,
    db: &Database,
    func_name: String,
    param_value_list: Vec<Expr>,
) -> Result<Val, EngineError> {
    let function = env.get_function(&func_name)?;
    function.evaluate_with_params(env, db, param_value_list)
}

fn eval_variable_def(
    env: &mut Env,
    db: &Database,
    name: String,
    value: Expr,
) -> Result<Val, EngineError> {
    let val = eval_expr(env, db, value)?;
    env.define_variable(&name, val)
}

fn eval_variable_assign(
    env: &mut Env,
    db: &Database,
    name: String,
    value: Expr,
) -> Result<Val, EngineError> {
    let val = eval_expr(env, db, value)?;
    env.set_variable(&name, val)
}

fn eval_if_stmt(env: &mut Env, db: &Database, if_stmt: If) -> Result<Val, EngineError> {
    let evaled_cond = eval_expr(env, db, if_stmt.expr)?;
    match evaled_cond.is_true()? {
        true => eval_code_block(env, db, if_stmt.body.stmts),
        false => match if_stmt.else_branch {
            Some(else_branch) => match *else_branch {
                ElseBranch::ElseIf(else_if_stmt) => eval_stmt(env, db, Stmt::ElseIf(else_if_stmt)),
                ElseBranch::Else(else_stmt) => eval_stmt(env, db, Stmt::Else(else_stmt)),
            },
            None => Ok(Val::Unit),
        },
    }
}

fn eval_unary_expr(
    env: &mut Env,
    db: &Database,
    op: UnaryOp,
    expr: Expr,
) -> Result<Val, EngineError> {
    let val = eval_expr(env, db, expr)?;
    match op {
        UnaryOp::Neg => val.neg(),
        UnaryOp::Not => val.not(),
    }
}

fn eval_binary_expr(
    env: &mut Env,
    db: &Database,
    op: BinaryOp,
    lhs: ExprIdx,
    rhs: ExprIdx,
) -> Result<Val, EngineError> {
    let lhs_val = eval_expr(env, db, db.exprs.index(lhs).to_owned())?;
    let rhs_val = eval_expr(env, db, db.exprs.index(rhs).to_owned())?;
    match op {
        BinaryOp::Add => lhs_val.add(rhs_val),
        BinaryOp::Sub => lhs_val.sub(rhs_val),
        BinaryOp::Mul => lhs_val.mul(rhs_val),
        BinaryOp::Div => lhs_val.div(rhs_val),
        BinaryOp::ComparisonEquals => lhs_val.equals(rhs_val),
        BinaryOp::GreaterThan => lhs_val.greater_than(rhs_val),
        BinaryOp::GreaterThanOrEqual => lhs_val.greater_than_or_eq(rhs_val),
        BinaryOp::LessThan => lhs_val.less_than(rhs_val),
        BinaryOp::LessThanOrEqual => lhs_val.less_than_or_eq(rhs_val),
        BinaryOp::Or => lhs_val.or(rhs_val),
        BinaryOp::And => lhs_val.and(rhs_val),
    }
}
