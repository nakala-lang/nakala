use crate::{env::Env, error::RuntimeError, val::Value};
use ast::expr::*;

pub(crate) fn eval_expr(expr: Expression, env: &mut Env) -> Result<Value, RuntimeError> {
    match expr.expr {
        Expr::Bool(..) | Expr::Int(..) | Expr::Float(..) | Expr::String(..) | Expr::Null => {
            Ok(expr.into())
        }
        Expr::Variable(..) => eval_variable_expr(expr, env),
        _ => todo!("{:#?} nyi", expr),
    }
}

fn eval_variable_expr(expr: Expression, env: &mut Env) -> Result<Value, RuntimeError> {
    if let Expr::Variable(name) = expr.expr {
        env.get(&name)
    } else {
        panic!("ICE: eval_variable_expr should only be called with Expr::Variable");
    }
}
