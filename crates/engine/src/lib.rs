use hir::{BinaryOp, Database, Expr, ExprIdx, Stmt};
use std::ops::Index;

pub mod env;
pub mod error;
pub mod val;

use env::Env;
use error::EngineError;
use val::Val;

pub fn eval(env: &mut Env, hir: (Database, Vec<Stmt>)) -> Result<Val, EngineError> {
    let (db, stmts) = hir;

    let first_stmt = stmts.get(0).unwrap();
    match first_stmt {
        Stmt::Expr(expr) => eval_expr(&env, &db, expr.to_owned()),
        Stmt::VariableDef { name, value } => {
            eval_variable_def(env, &db, name.to_string(), value.to_owned())
        }
        _ => todo!(),
    }
}

fn eval_expr(env: &Env, db: &Database, expr: Expr) -> Result<Val, EngineError> {
    match expr {
        Expr::Binary { op, lhs, rhs } => eval_binary_expr(&env, &db, op, lhs, rhs),
        Expr::Literal { n } => Ok(Val::Number(n.into())),
        Expr::VariableRef { var } => env.get_binding(var.to_string()),
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
    env.set_binding(name, val)
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
