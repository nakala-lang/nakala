pub mod env;
pub mod error;
mod expr;
pub mod val;

use crate::env::{Environment, ScopeId};
use crate::error::RuntimeError;
use crate::expr::eval_expr;
use ast::{expr::*, op::*, stmt::*, ty::*};
use meta::trace;
use parser::Parse;
use val::{Val, Value};

pub fn interpret(parse: Parse, env: Option<&mut Environment>) -> Result<(), RuntimeError> {
    let mut new_env = Environment::new();
    let env = env.unwrap_or(&mut new_env);

    for _stmt in parse.stmts {
        eval_stmt(_stmt, env, 0)?;
    }

    trace!(format!("{:?}", env));

    Ok(())
}

fn eval_stmt(stmt: Statement, env: &mut Environment, scope: ScopeId) -> Result<(), RuntimeError> {
    match stmt.stmt {
        Stmt::Expr(expr) => {
            eval_expr(expr, env, scope)?;
        }
        Stmt::Variable { .. } => {
            eval_variable(stmt, env, scope)?;
        }
        Stmt::Print(expr) => {
            println!("{}", eval_expr(expr, env, scope)?);
        }
        Stmt::Block(..) => {
            eval_block(stmt, env, scope)?;
        }
        Stmt::Function(..) => eval_func_decl(stmt, env, scope)?,
        Stmt::Class(..) => eval_class_decl(stmt, env, scope)?,
        _ => todo!("{:#?} nyi", stmt),
    }

    Ok(())
}

fn eval_variable(stmt: Statement, env: &mut Environment, scope: ScopeId) -> Result<(), RuntimeError> {
    if let Stmt::Variable {
        name: binding,
        expr,
    } = stmt.stmt
    {
        let var_name = binding.name.item;

        let mut val = Value::null(); 

        if let Some(expr) = expr {
            val = eval_expr(expr, env, scope)?;
        }

        env.define(scope, var_name, val)?;

        Ok(())
    } else {
        panic!("ICE: eval_variable should only be called with Stmt::Variable");
    }
}

fn eval_block(stmt: Statement, env: &mut Environment, scope: ScopeId) -> Result<Value, RuntimeError> {
    let mut ret_val = Value::null();

    if let Stmt::Block(stmts) = stmt.stmt {
        for _stmt in stmts {
            if let Stmt::Return(ret_expr) = _stmt.stmt {
                if let Some(expr) = ret_expr {
                    ret_val = eval_expr(expr, env, scope)?;
                }

                return Ok(ret_val);
            }

            eval_stmt(_stmt, env, scope)?;
        }

        Ok(ret_val)
    } else {
        panic!("ICE: eval_block should only be called with Stmt::Block");
    }
}

fn eval_func_decl(stmt: Statement, env: &mut Environment, scope: ScopeId) -> Result<(), RuntimeError> {
    if let Stmt::Function(func) = stmt.stmt {
        let func_name = func.name.item.clone();
        env.define(
            scope,
            func_name,
            Value {
                val: Val::Function {
                    func,
                    closure: scope
                },
                span: stmt.span,
            },
        )?;

        Ok(())
    } else {
        panic!("ICE: eval_func should only be called with Stmt::Function");
    }
}

fn eval_class_decl(stmt: Statement, env: &mut Environment, scope: ScopeId) -> Result<(), RuntimeError> {
    if let Stmt::Class(class) = stmt.stmt {
        let class_name = class.name.item.clone();

        env.define(
            scope,
            class_name,
            Value {
                val: Val::Class(class),
                span: stmt.span
            }
        )?;

        Ok(())
    } else {
        panic!("ICE: eval_class_decl should only be called with Stmt::Class");
    }
}