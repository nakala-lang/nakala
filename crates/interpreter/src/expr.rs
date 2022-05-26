use crate::{env::Env, error::RuntimeError, val::{Val, Value}};
use meta::Span;
use ast::expr::*;

pub(crate) fn eval_expr(expr: Expression, env: &mut Env) -> Result<Value, RuntimeError> {
    match expr.expr {
        Expr::Bool(..) | Expr::Int(..) | Expr::Float(..) | Expr::String(..) | Expr::Null => {
            Ok(expr.into())
        }
        Expr::Variable(..) => eval_variable_expr(expr, env),
        Expr::Assign { .. } => eval_assign_expr(expr, env),
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

fn eval_assign_expr(expr: Expression, env: &mut Env) -> Result<Value, RuntimeError> {
    if let Expr::Assign { name, rhs } = expr.expr {
        let val = eval_expr(*rhs, env)?;
        
        env.assign(name.item, val)?;

        Ok(Value {
            val: Val::Null,
            span: Span::garbage()
        })
    } else {
        panic!("ICE: eval_assign_expr should only be called with Expr::Assign");
    }
}
