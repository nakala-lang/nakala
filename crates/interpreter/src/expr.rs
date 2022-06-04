use crate::{
    env::{Environment, ScopeId},
    error::RuntimeError,
    value::{Callable, Val, Value},
};
use ast::{expr::*, op::Op};
use meta::{Span, Spanned};

pub(crate) fn eval_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    match expr.expr {
        Expr::Bool(..) | Expr::Int(..) | Expr::Float(..) | Expr::String(..) | Expr::Null => {
            Ok(expr.into())
        }
        Expr::Variable(..) => eval_variable_expr(expr, env, scope),
        Expr::Assign { .. } => eval_assign_expr(expr, env, scope),
        Expr::Call { .. } => eval_call_expr(expr, env, scope),
        Expr::Binary { .. } => eval_binary_expr(expr, env, scope),
        Expr::Logical { .. } => eval_logical_expr(expr, env, scope),
        Expr::Get { .. } => eval_get_expr(expr, env, scope),
        Expr::Set { .. } => eval_set_expr(expr, env, scope),
        Expr::This => eval_this_expr(expr, env, scope),
        _ => todo!("{:#?} nyi", expr),
    }
}

fn eval_variable_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::Variable(name) = expr.expr {
        env.get(
            scope,
            &Spanned {
                item: name,
                span: expr.span,
            },
        )
    } else {
        panic!("ICE: eval_variable_expr should only be called with Expr::Variable");
    }
}

fn eval_assign_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::Assign { name, rhs } = expr.expr {
        let val = eval_expr(*rhs, env, scope)?;

        env.assign(scope, name, val)?;

        Ok(Value::null())
    } else {
        panic!("ICE: eval_assign_expr should only be called with Expr::Assign");
    }
}

fn eval_call_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::Call { callee, args, .. } = expr.expr {
        let val = eval_expr(*callee, env, scope)?;

        match val.val {
            Val::Function(func) => func.call(args, env, scope),
            Val::Class(class) => class.call(args, env, scope),
            Val::Builtin(builtin) => builtin.call(args, env, scope),
            _ => panic!("ICE: can only call t"),
        }
    } else {
        panic!("ICE: eval_call expr should only be called with Expr::Call");
    }
}

fn eval_binary_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::Binary { lhs, op, rhs } = expr.expr {
        let lhs = eval_expr(*lhs, env, scope)?;
        let rhs = eval_expr(*rhs, env, scope)?;

        match op.op {
            Op::Add => lhs.add(op, &rhs),
            Op::Sub => lhs.sub(op, &rhs),
            Op::Mul => lhs.mul(op, &rhs),
            Op::Div => lhs.div(op, &rhs),
            Op::Equals => lhs.eq(&rhs),
            Op::NotEquals => lhs.neq(&rhs),
            Op::LessThanEquals => lhs.lte(op, &rhs),
            Op::GreaterThan => lhs.gt(op, &rhs),
            Op::Or | Op::And => {
                unreachable!("ICE: logical binary expressions should be parsed as such")
            }
            _ => todo!("unsupported operation {:#?}", op),
        }
    } else {
        panic!("ICE: eval_binary_expr should only be called with Expr::Binary");
    }
}

fn eval_get_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::Get { object, name } = expr.expr {
        let obj = eval_expr(*object, env, scope)?;

        let instance = env.get_instance(obj.as_instance()?)?;

        // If property is a function, bind 'this'
        let mut prop = instance.get_property(&name.item)?;
        if let Val::Function(..) = prop.val {
            prop.bind_this(env, obj)?;
        }

        Ok(prop)
    } else {
        panic!("ICE: eval_get_expr should only be called with Expr::Get");
    }
}

fn eval_set_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::Set { object, name, rhs } = expr.expr {
        let obj = eval_expr(*object, env, scope)?;
        let val = eval_expr(*rhs, env, scope)?;

        let instance = env.get_instance(obj.as_instance()?)?;
        instance.set_property(name.item, val)
    } else {
        panic!("ICE: eval_set_expr should only be called with Expr::Set");
    }
}

fn eval_logical_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::Logical { lhs, op, rhs } = expr.expr {
        let span = Span::combine(&[lhs.span, rhs.span]);

        let lhs = eval_expr(*lhs, env, scope)?;

        match op.op {
            Op::And => {
                // Short circuit if false
                if !lhs.as_bool()? {
                    return Ok((false, span).into());
                }

                let rhs = eval_expr(*rhs, env, scope)?;
                lhs.and(op, &rhs)
            }
            Op::Or => {
                // Short circuit if true
                if lhs.as_bool()? {
                    return Ok((true, span).into());
                }

                let rhs = eval_expr(*rhs, env, scope)?;
                lhs.or(op, &rhs)
            }
            _ => unreachable!(
                "ICE: logical expressions was given non logical operator {:?}",
                op.op
            ),
        }
    } else {
        panic!("ICE: eval_logical_expr should only be called with Expr::Logical");
    }
}

fn eval_this_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::This = expr.expr {
        let t: Spanned<String> = Spanned {
            item: String::from("this"),
            span: expr.span,
        };

        env.get(scope, &t)
    } else {
        panic!("ICE: eval_this_expr should only be called with Expr::This");
    }
}
