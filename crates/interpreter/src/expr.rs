use crate::{env::Env, error::RuntimeError, eval_block, val::{Val, Value}};
use ast::{expr::*, stmt::Function};
use meta::Span;

pub(crate) fn eval_expr(expr: Expression, env: &mut Env) -> Result<Value, RuntimeError> {
    match expr.expr {
        Expr::Bool(..) | Expr::Int(..) | Expr::Float(..) | Expr::String(..) | Expr::Null => {
            Ok(expr.into())
        }
        Expr::Variable(..) => eval_variable_expr(expr, env),
        Expr::Assign { .. } => eval_assign_expr(expr, env),
        Expr::Call { .. } => eval_call_expr(expr, env),
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

        Ok(Value::null())
    } else {
        panic!("ICE: eval_assign_expr should only be called with Expr::Assign");
    }
}

fn eval_call_expr(expr: Expression, env: &mut Env) -> Result<Value, RuntimeError> {
    if let Expr::Call {
        callee,
        paren,
        args,
    } = expr.expr
    {
        match callee.expr {
            Expr::Variable(name) => {
                let entry = env.get(&name)?;
                match entry.val {
                    Val::Function(func) => eval_func_call(func, paren, args, env),
                    _ => panic!("ICE: can only call functions"),
                }
            }
            _ => panic!("ICE: parser gave non callable expr for Expr::Call"),
        }
    } else {
        panic!("ICE: eval_call expr should only be called with Expr::Call");
    }
}

fn eval_func_call(func: Function, paren: Span, args: Vec<Expression>, env: &mut Env) -> Result<Value, RuntimeError> {
    if func.params.len() != args.len() {
        todo!("parity mismatch");
    }

    env.begin_scope();
    for (param, arg) in func.params.into_iter().zip(args.into_iter()) {
        let val = eval_expr(arg, env)?;
        env.define(param.name.item.clone(), val)?;
    }

    let ret_val = eval_block(*func.body, env)?;
    env.end_scope();

    Ok(ret_val)
}
