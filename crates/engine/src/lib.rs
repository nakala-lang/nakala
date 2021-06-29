use hir::{BinaryOp, Database, Expr, ExprIdx, Stmt};
use std::ops::Index;

pub mod error;
pub mod val;

use error::EngineError;
use val::Val;

pub fn eval(hir: (Database, Vec<Stmt>)) -> Result<Val, EngineError> {
    let (db, stmts) = hir;
    let first_stmt = stmts.get(0).unwrap();
    match first_stmt {
        Stmt::Expr(expr) => eval_expr(&db, expr.to_owned()),
        _ => todo!(),
    }
}

fn eval_expr(db: &Database, expr: Expr) -> Result<Val, EngineError> {
    match expr {
        Expr::Binary { op, lhs, rhs } => eval_binary_expr(&db, op, lhs, rhs),
        Expr::Literal { n } => Ok(Val::Number(n.into())),
        _ => Err(EngineError::InvalidExpression(expr)),
    }
}

fn eval_binary_expr(
    db: &Database,
    op: BinaryOp,
    lhs: ExprIdx,
    rhs: ExprIdx,
) -> Result<Val, EngineError> {
    let lhs_val = eval_expr(&db, db.exprs.index(lhs).to_owned())?;
    let rhs_val = eval_expr(&db, db.exprs.index(rhs).to_owned())?;
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
