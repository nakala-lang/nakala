use std::collections::HashMap;

use crate::{env::{Environment, ScopeId}, error::RuntimeError, eval_block, val::{Val, Value}};
use ast::{expr::*, op::{Op, Operator}, stmt::{Class, Function}};
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
        Expr::Get { .. } => eval_get_expr(expr, env, scope),
        Expr::Set { .. } => eval_set_expr(expr, env, scope),
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
        let val = eval_expr(*callee, env, scope)?;

        match val.val {
            Val::Function { func, closure } => eval_func_call(func, paren, args, env, closure),
            Val::Class(class) => eval_class_instantiation(class, env, scope),
            _ => panic!("ICE: can only call functions"),
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

fn eval_class_instantiation(class: Class, env: &mut Environment, scope: ScopeId) -> Result<Value, RuntimeError> {
    let ret_val = Value {
        span: class.name.span,
        val: Val::Instance {
            class,
            fields: HashMap::default(),
        }
    };

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

fn eval_get_expr(expr: Expression, env: &mut Environment, scope: ScopeId) -> Result<Value, RuntimeError> {
    if let Expr::Get { object, name } = expr.expr {
        let instance = eval_expr(*object, env, scope)?;
        instance.get_property(&name.item)
    } else {
        panic!("ICE: eval_get_expr should only be called with Expr::Get");
    }
}

fn eval_set_expr(expr: Expression, env: &mut Environment, scope: ScopeId) -> Result<Value, RuntimeError> {
    if let Expr::Set { object, name, rhs } = expr.expr {
        let mut instance = eval_expr(*object, env, scope)?;
        let val = eval_expr(*rhs, env, scope)?;

        println!("set_expr: setting {:?} to {:?}", name.item.clone(), val.clone());

        instance.set_property(name.item, val)
    } else {
        panic!("ICE: eval_set_expr should only be called with Expr::Set");
    }
}
