use crate::{env::{Environment, ScopeId}, error::RuntimeError, eval_block, val::{Val, Value}};
use ast::{expr::*, op::{Op, Operator}, stmt::Function};
use meta::Span;

pub(crate) fn eval_expr(expr: Expression, env: &mut Environment, scope: ScopeId) -> Result<Value, RuntimeError> {
    match expr.expr {
        Expr::Bool(..) | Expr::Int(..) | Expr::Float(..) | Expr::String(..) | Expr::Null => {
            Ok(expr.into())
        }
        Expr::Variable(..) => eval_variable_expr(expr, env, scope),
        Expr::Assign { .. } => eval_assign_expr(expr, env, scope),
        Expr::Call { .. } => eval_call_expr(expr, env, scope),
        Expr::Binary {.. } => eval_binary_expr(expr, env, scope),
        _ => todo!("{:#?} nyi", expr),
    }
}

fn eval_variable_expr(expr: Expression, env: &mut Environment, scope: ScopeId) -> Result<Value, RuntimeError> {
    if let Expr::Variable(name) = expr.expr {
        env.get(scope, &name)
    } else {
        panic!("ICE: eval_variable_expr should only be called with Expr::Variable");
    }
}

fn eval_assign_expr(expr: Expression, env: &mut Environment, scope: ScopeId) -> Result<Value, RuntimeError> {
    if let Expr::Assign { name, rhs } = expr.expr {
        let val = eval_expr(*rhs, env, scope)?;

        env.assign(scope, name.item, val)?;

        Ok(Value::null())
    } else {
        panic!("ICE: eval_assign_expr should only be called with Expr::Assign");
    }
}

fn eval_call_expr(expr: Expression, env: &mut Environment, scope: ScopeId) -> Result<Value, RuntimeError> {
    if let Expr::Call {
        callee,
        paren,
        args,
    } = expr.expr
    {
        match callee.expr {
            Expr::Variable(name) => {
                let entry = env.get(scope, &name)?;
                match entry.val {
                    Val::Function { func, closure } => eval_func_call(func, paren, args, env, closure),
                    _ => panic!("ICE: can only call functions"),
                }
            }
            _ => panic!("ICE: parser gave non callable expr for Expr::Call"),
        }
    } else {
        panic!("ICE: eval_call expr should only be called with Expr::Call");
    }
}

fn eval_func_call(func: Function, paren: Span, args: Vec<Expression>, env: &mut Environment, closure: ScopeId) -> Result<Value, RuntimeError> {
    if func.params.len() != args.len() {
        todo!("parity mismatch");
    }

    let new_scope = env.begin_scope(Some(closure));

    for (param, arg) in func.params.into_iter().zip(args.into_iter()) {
        let val = eval_expr(arg, env, new_scope)?;
        env.define(new_scope, param.name.item.clone(), val)?;
    }

    let ret_val = eval_block(*func.body, env, new_scope)?;

    Ok(ret_val)
}

fn eval_binary_expr(expr: Expression, env: &mut Environment, scope: ScopeId) -> Result<Value, RuntimeError> {
    if let Expr::Binary { lhs, op, rhs } = expr.expr {
        let lhs = eval_expr(*lhs, env, scope)?;
        let rhs = eval_expr(*rhs, env, scope)?;

        match op.op {
            Op::Add => lhs.add(op, &rhs),
            _ => todo!("unsupported operation {:#?}", op)
        }
    } else {
        panic!("ICE: eval_binary_expr should only be called with Expr::Binary");
    }
}
