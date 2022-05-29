pub mod env;
pub mod error;
mod expr;
pub mod val;

use crate::env::{Environment, EnvId};
use crate::error::RuntimeError;
use crate::expr::eval_expr;
use ast::{expr::*, op::*, stmt::*, ty::*};
use meta::Span;
use parser::Parse;
use val::{Val, Value};

pub fn interpret(parse: Parse, env: Option<&mut Environment>) -> Result<(), RuntimeError> {
    let mut new_env = Environment::new();
    let env = env.unwrap_or(&mut new_env);

    for _stmt in parse.stmts {
        eval_stmt(_stmt, env, 0)?;
    }

    Ok(())
}

fn eval_stmt(stmt: Statement, env: &mut Environment, env_id: EnvId) -> Result<(), RuntimeError> {
    match stmt.stmt {
        Stmt::Expr(expr) => {
            eval_expr(expr, env, env_id)?;
        }
        Stmt::Variable { .. } => {
            eval_variable(stmt, env, env_id)?;
        }
        Stmt::Print(expr) => {
            println!("{}", eval_expr(expr, env, env_id)?);
        }
        Stmt::Block(..) => {
            eval_block(stmt, env, env_id)?;
        }
        Stmt::Function(..) => eval_func_decl(stmt, env, env_id)?,
        _ => todo!("{:#?} nyi", stmt),
    }

    Ok(())
}

fn eval_variable(stmt: Statement, env: &mut Environment, env_id: EnvId) -> Result<(), RuntimeError> {
    if let Stmt::Variable {
        name: binding,
        expr,
    } = stmt.stmt
    {
        let var_name = binding.name.item;

        let mut val = Value::null(); 

        if let Some(expr) = expr {
            val = eval_expr(expr, env, env_id)?;
        }

        env.define(env_id, var_name, val)?;

        Ok(())
    } else {
        panic!("ICE: eval_variable should only be called with Stmt::Variable");
    }
}

fn eval_block(stmt: Statement, env: &mut Environment, env_id: EnvId) -> Result<Value, RuntimeError> {
    let mut ret_val = Value::null();

    if let Stmt::Block(stmts) = stmt.stmt {
        for _stmt in stmts {
            if let Stmt::Return(ret_expr) = _stmt.stmt {
                if let Some(expr) = ret_expr {
                    ret_val = eval_expr(expr, env, env_id)?;
                }

                return Ok(ret_val);
            }

            eval_stmt(_stmt, env, env_id)?;
        }

        Ok(ret_val)
    } else {
        panic!("ICE: eval_block should only be called with Stmt::Block");
    }
}

fn eval_func_decl(stmt: Statement, env: &mut Environment, env_id: EnvId) -> Result<(), RuntimeError> {
    if let Stmt::Function(func) = stmt.stmt {
        let func_name = func.name.item.clone();
        env.define(
            env_id,
            func_name,
            Value {
                val: Val::Function {
                    func,
                    closure: env_id
                },
                span: stmt.span,
            },
        )?;

        Ok(())
    } else {
        panic!("ICE: eval_func should only be called with Stmt::Function");
    }
}
