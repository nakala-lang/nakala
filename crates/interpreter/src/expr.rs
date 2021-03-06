use crate::{
    env::{Environment, ScopeId},
    error::RuntimeError,
    value::{Callable, Indexible, Val, Value},
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
        Expr::List(..) => eval_list_expr(expr, env, scope),
        Expr::IndexGet { .. } => eval_index_get_expr(expr, env, scope),
        Expr::IndexSet { .. } => eval_index_set_expr(expr, env, scope),
        Expr::ListShorthand { .. } => eval_list_shorthand_expr(expr, env, scope),
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
        let callee_span = callee.span;
        let val = eval_expr(*callee, env, scope)?;

        match val.val {
            Val::Function(func) => func.call(callee_span, args, env, scope),
            Val::Class(class) => class.call(callee_span, args, env, scope),
            Val::Builtin(builtin) => builtin.call(callee_span, args, env, scope),
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
            Op::Add => lhs.add(env, op, &rhs),
            Op::Sub => lhs.sub(op, &rhs),
            Op::Mul => lhs.mul(op, &rhs),
            Op::Div => lhs.div(op, &rhs),
            Op::Equals => lhs.eq(&rhs),
            Op::NotEquals => lhs.neq(&rhs),
            Op::LessThanEquals => lhs.lte(op, &rhs),
            Op::GreaterThan => lhs.gt(op, &rhs),
            Op::GreaterThanEquals => lhs.gte(op, &rhs),
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
        if let Val::Instance { .. } = obj.val {
            let instance = env.get_instance(obj.as_instance()?)?;

            // If property is a function, bind 'this'
            let mut prop = instance.get_property(&name.item)?;
            if let Val::Function(..) = prop.val {
                prop.bind_this(env, obj)?;
            }

            Ok(prop)
        } else {
            let class = obj.as_class()?;
            if let Some(entry) = class.statics.get(&name.item) {
                Ok(entry.clone())
            } else {
                Err(RuntimeError::UndefinedClassProperty(
                    class.class.name.span.source_id,
                    class.class.name.span.into(),
                    name.item.to_string(),
                ))
            }
        }
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

fn eval_list_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::List(list) = expr.expr {
        let mut vals = vec![];
        for val in list.into_iter() {
            vals.push(eval_expr(val, env, scope)?);
        }

        Ok(env.new_list(vals, expr.ty.clone()))
    } else {
        panic!("ICE: eval_list_expr should only be called with Expr::List");
    }
}

fn eval_index_get_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::IndexGet { lhs, index } = expr.expr {
        let lhs = eval_expr(*lhs, env, scope)?;
        let index = eval_expr(*index, env, scope)?;

        match lhs.val {
            Val::List { id } => env.get_list(id).get(index.as_int()?.try_into().unwrap()),
            _ => panic!("ICE: can only index Lists"),
        }
    } else {
        panic!("ICE: eval_index_get_expr should only be called with Expr::Index");
    }
}

fn eval_index_set_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::IndexSet { lhs, index, rhs } = expr.expr {
        let lhs = eval_expr(*lhs, env, scope)?;
        let index = eval_expr(*index, env, scope)?;
        let rhs = eval_expr(*rhs, env, scope)?;

        let lhs = match lhs.val {
            Val::List { id } => env.get_list(id),
            _ => panic!("ICE: can only index Lists"),
        };

        lhs.set(index.as_int()?.try_into().unwrap(), rhs)?;

        Ok(Value::null())
    } else {
        panic!("ICE: eval_index_set_expr should only be called with Expr::IndexSet");
    }
}

fn eval_list_shorthand_expr(
    expr: Expression,
    env: &mut Environment,
    scope: ScopeId,
) -> Result<Value, RuntimeError> {
    if let Expr::ListShorthand { value, count } = expr.expr {
        let value = eval_expr(*value, env, scope)?;
        let count = eval_expr(*count, env, scope)?.as_int()?;

        let mut vals: Vec<Value> = Vec::with_capacity(count.try_into().unwrap());
        for _ in 0..count {
            vals.push(value.clone());
        }

        Ok(env.new_list(vals, value.ty))
    } else {
        panic!("ICE: eval_list_shorthand_expr should only be called with Expr::ListShorthand");
    }
}
