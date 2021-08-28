use hir::{
    BinaryOp, CodeBlock, Database, ElseBranch, Expr, ExprIdx, ForLoop, FunctionDef, Hir, If,
    Return, Stmt, UnaryOp, VariableAssign, VariableDef,
};
use std::ops::Index;

pub mod builtins;
pub mod class;
pub mod env;
pub mod error;
pub mod func;
pub mod val;

use class::{Class, ClassDef};
use env::Env;
use error::EngineError;
use val::Val;

use crate::builtins::dispatch_builtin;

pub fn eval(env: &mut Env, hir: Hir) -> Result<Val, EngineError> {
    let db = hir.db;
    let stmts = hir.stmts;
    let mut return_val = Val::Unit;

    for stmt in stmts {
        return_val = eval_stmt(env, &db, stmt)?;
    }

    Ok(return_val)
}

fn eval_stmt(env: &mut Env, db: &Database, stmt: Stmt) -> Result<Val, EngineError> {
    match stmt {
        Stmt::Expr(expr) => eval_expr(env, db, expr),
        Stmt::VariableDef(VariableDef { name, value }) => {
            eval_variable_def(env, db, name.to_string(), value)
        }
        Stmt::VariableAssign(VariableAssign { name, value }) => {
            eval_variable_assign(env, db, name.to_string(), value)
        }
        Stmt::FunctionDef(func_def) => eval_function_def(env, db, func_def),
        Stmt::If(if_stmt) => eval_if_stmt(env, db, if_stmt),
        Stmt::ElseIf(else_if) => eval_if_stmt(env, db, else_if.if_stmt),
        Stmt::Else(else_stmt) => eval_code_block(env, db, else_stmt.body.stmts),
        Stmt::Return(return_stmt) => eval_return(env, db, return_stmt),
        Stmt::ClassDef(class_def) => eval_class_def(env, db, class_def),
        Stmt::ForLoop(for_loop) => eval_for_loop(env, db, for_loop),
    }
}

fn eval_code_block(env: &mut Env, db: &Database, stmts: Vec<Stmt>) -> Result<Val, EngineError> {
    let mut block_env = Env::new(Some(Box::new(env.clone())));
    let mut return_val = Val::Unit;

    for stmt in stmts {
        if let Stmt::Return(r) = stmt.clone() {
            block_env.propagate_enclosing_env_changes(env);
            return Err(EngineError::EarlyReturn {
                value: eval_return(&mut block_env, db, r)?,
            });
        } else {
            return_val = eval_stmt(&mut block_env, db, stmt)?;
        }
    }

    block_env.propagate_enclosing_env_changes(env);

    Ok(return_val)
}

fn eval_function_def(
    env: &mut Env,
    db: &Database,
    func_def: FunctionDef,
) -> Result<Val, EngineError> {
    env.set_function(func::Function::new(func_def, db.clone()))
}

fn eval_expr(env: &mut Env, db: &Database, expr: Expr) -> Result<Val, EngineError> {
    match expr {
        Expr::Binary { op, lhs, rhs } => eval_binary_expr(env, db, op, lhs, rhs),
        Expr::Number { n } => Ok(Val::Number(n)),
        Expr::String { s } => Ok(Val::String(s)),
        Expr::Boolean { b } => Ok(Val::Boolean(b)),
        Expr::VariableRef { var } => env.get_variable(&var.to_string()),
        Expr::Unary { op, expr } => eval_unary_expr(env, db, op, db.exprs.index(expr).to_owned()),
        Expr::CodeBlock(CodeBlock { stmts }) => eval_code_block(env, db, stmts),
        Expr::FunctionCall {
            name,
            param_value_list,
        } => eval_function_call(env, db, name, param_value_list),
        Expr::List { items } => eval_list(env, db, items),
        Expr::IndexOp { ident, index } => eval_index_op(env, db, ident, *index),
        Expr::ClassCreate {
            name,
            param_value_list,
        } => eval_class_create(env, db, name, param_value_list),
        Expr::Missing => {
            unreachable!("Missing tokens will get caught before they reach the engine")
        }
    }
}

fn eval_function_call(
    env: &mut Env,
    db: &Database,
    func_name: String,
    param_value_list: Vec<Expr>,
) -> Result<Val, EngineError> {
    if builtins::BUILTINS.contains(&func_name.as_str()) {
        let mut values = Vec::new();
        for val in param_value_list {
            values.push(eval_expr(env, db, val)?);
        }
        dispatch_builtin(func_name.as_str(), values)
    } else {
        let function = env.get_function(&func_name)?;
        function.evaluate_with_params(env, db, param_value_list)
    }
}

fn eval_list(env: &mut Env, db: &Database, items: Vec<Expr>) -> Result<Val, EngineError> {
    let mut evaluated_items = Vec::new();
    for expr in items {
        evaluated_items.push(eval_expr(env, db, expr)?);
    }

    Ok(Val::List(evaluated_items))
}

fn eval_index_op(
    env: &mut Env,
    db: &Database,
    ident: String,
    index_expr: Expr,
) -> Result<Val, EngineError> {
    let val = env.get_variable(ident.as_str())?;
    let index = eval_expr(env, db, index_expr)?;
    val.index(index)
}

fn eval_variable_def(
    env: &mut Env,
    db: &Database,
    name: String,
    value: Expr,
) -> Result<Val, EngineError> {
    let val = eval_expr(env, db, value)?;
    env.define_variable(&name, val)
}

fn eval_variable_assign(
    env: &mut Env,
    db: &Database,
    name: String,
    value: Expr,
) -> Result<Val, EngineError> {
    let val = eval_expr(env, db, value)?;
    env.set_variable(&name, val).map(|_| Val::Unit)
}

fn eval_if_stmt(env: &mut Env, db: &Database, if_stmt: If) -> Result<Val, EngineError> {
    let evaled_cond = eval_expr(env, db, if_stmt.expr)?;
    match evaled_cond.is_true()? {
        true => eval_code_block(env, db, if_stmt.body.stmts),
        false => match if_stmt.else_branch {
            Some(else_branch) => match *else_branch {
                ElseBranch::ElseIf(else_if_stmt) => eval_stmt(env, db, Stmt::ElseIf(else_if_stmt)),
                ElseBranch::Else(else_stmt) => eval_stmt(env, db, Stmt::Else(else_stmt)),
            },
            None => Ok(Val::Unit),
        },
    }
}

fn eval_for_loop(env: &mut Env, db: &Database, for_loop: ForLoop) -> Result<Val, EngineError> {
    let item = for_loop.item.as_str();
    let collection: Vec<Val> = match eval_expr(env, db, for_loop.collection)? {
        Val::List(items) => items,
        Val::String(s) => s
            .split("")
            .into_iter()
            .map(|x| Val::String(String::from(x)))
            .collect(),
        x => return Err(EngineError::NonIterableValue { x }),
    };

    let stmts = for_loop.body.stmts;
    let mut loop_env = Env::new(Some(Box::new(env.clone())));
    loop_env.define_variable(item, Val::Unit)?;

    for collection_item in collection {
        loop_env.set_variable(item, collection_item)?;
        eval_code_block(&mut loop_env, db, stmts.clone())?;
    }

    loop_env.propagate_enclosing_env_changes(env);

    Ok(Val::Unit)
}

fn eval_unary_expr(
    env: &mut Env,
    db: &Database,
    op: UnaryOp,
    expr: Expr,
) -> Result<Val, EngineError> {
    let val = eval_expr(env, db, expr)?;
    match op {
        UnaryOp::Neg => val.neg(),
        UnaryOp::Not => val.not(),
    }
}

fn eval_binary_expr(
    env: &mut Env,
    db: &Database,
    op: BinaryOp,
    lhs: ExprIdx,
    rhs: ExprIdx,
) -> Result<Val, EngineError> {
    let lhs_val = eval_expr(env, db, db.exprs.index(lhs).to_owned())?;
    let rhs_val = eval_expr(env, db, db.exprs.index(rhs).to_owned())?;
    match op {
        BinaryOp::Add => lhs_val.add(rhs_val),
        BinaryOp::Sub => lhs_val.sub(rhs_val),
        BinaryOp::Mul => lhs_val.mul(rhs_val),
        BinaryOp::Div => lhs_val.div(rhs_val),
        BinaryOp::ComparisonEquals => lhs_val.equals(rhs_val),
        BinaryOp::GreaterThan => lhs_val.greater_than(rhs_val),
        BinaryOp::GreaterThanOrEqual => lhs_val.greater_than_or_eq(rhs_val),
        BinaryOp::LessThan => lhs_val.less_than(rhs_val),
        BinaryOp::LessThanOrEqual => lhs_val.less_than_or_eq(rhs_val),
        BinaryOp::Or => lhs_val.or(rhs_val),
        BinaryOp::And => lhs_val.and(rhs_val),
    }
}

fn eval_return(env: &mut Env, db: &Database, ret: Return) -> Result<Val, EngineError> {
    if let Some(expr) = ret.value {
        eval_expr(env, db, expr)
    } else {
        Ok(Val::Unit)
    }
}

fn eval_class_def(
    env: &mut Env,
    db: &Database,
    class_def: hir::ClassDef,
) -> Result<Val, EngineError> {
    env.define_class(ClassDef::new(class_def, db))
        .map(|_| Val::Unit)
}

fn eval_class_create(
    env: &mut Env,
    db: &Database,
    name: String,
    param_value_list: Vec<Expr>,
) -> Result<Val, EngineError> {
    let class_def = env.get_class_def(name.as_str())?;

    let expected = class_def.fields.len();
    let actual = param_value_list.len();
    if expected != actual {
        return Err(EngineError::ClassCreateMismatchedParameterCount {
            name,
            actual,
            expected,
        });
    }

    let mut init_value_list: Vec<Val> = vec![];
    for expr in param_value_list {
        init_value_list.push(eval_expr(env, db, expr)?);
    }

    Ok(Val::Class(Class::new(class_def, init_value_list)?))
}
