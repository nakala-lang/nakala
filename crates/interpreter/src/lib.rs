pub mod error;
pub mod env;
pub mod val;
mod expr;

use ast::{stmt::*, expr::*, op::*, ty::*};
use meta::Span;
use parser::Parse;
use val::{Val, Value};
use crate::env::Env;
use crate::error::RuntimeError;
use crate::expr::eval_expr;

pub fn interpret(parse: Parse, env: Option<&mut Env>) -> Result<(), RuntimeError> {
    let mut new_env = Env::new();
    let env = env.unwrap_or(&mut new_env);

    for _stmt in parse.stmts {
        eval_stmt(_stmt, env)?;
    }
    Ok(())   
}

fn eval_stmt(stmt: Statement, env: &mut Env) -> Result<(), RuntimeError> {
    match stmt.stmt {
        Stmt::Expr(expr) => {
            eval_expr(expr, env)?;
        }
        Stmt::Variable { .. } => {
            eval_variable(stmt, env)?;
        }
        Stmt::Print(expr) => {
            print!("{}", eval_expr(expr, env)?);
        }
        Stmt::Block(stmts) => {
            for block_stmt in stmts {
                eval_stmt(block_stmt, env)?;
            }
        }
        _ => todo!("{:#?} nyi", stmt)   
    }

    Ok(())
}

fn eval_variable(stmt: Statement, env: &mut Env) -> Result<(), RuntimeError> {
    if let Stmt::Variable { name: binding, expr } = stmt.stmt {
        let var_name = binding.name.item;

        let mut val = Value {
            val: Val::Null,
            span: Span::garbage()
        };

        if let Some(expr) = expr {
            val = eval_expr(expr, env)?;
        }

        env.define(var_name, val);

        Ok(())
    } else {
        panic!("ICE: eval_variable should only be called with Stmt::Variable");
    }
}
