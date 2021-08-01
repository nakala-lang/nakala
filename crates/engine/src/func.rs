use crate::env::Env;
use crate::{error::EngineError, val::Val};
use hir::{CodeBlock, Database, Expr, FunctionDef};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    name: String,
    param_list: Vec<String>,
    body: CodeBlock,
    // we need to keep the db of the original block around as well
    body_db: Database,
}

impl Function {
    pub fn new(func_def: FunctionDef, db: Database) -> Self {
        Function {
            name: func_def.name.to_string(),
            param_list: func_def
                .param_ident_list
                .into_iter()
                .map(|p| p.to_string())
                .collect(),
            body: func_def.body,
            body_db: db,
        }
    }

    pub fn evaluate_with_params(
        &self,
        env: &Env,
        db: &Database,
        params: Vec<Expr>,
    ) -> Result<Val, EngineError> {
        if params.len() != self.param_list.len() {
            return Err(EngineError::MismatchedParameterCount {
                actual: params.len(),
                expected: self.param_list.len(),
            });
        }

        // clone the current environment, and also evaluate parameters
        let mut cloned_env = env.clone();

        for (index, param) in params.into_iter().enumerate() {
            // param names could overlap with scope names, so we need to rename any overlaps
            let param_name = self.param_list.get(index).unwrap().to_string();

            // Append a `_` every time to the overlapping outside of the function
            while let Ok(overlapping_outside_variable) = cloned_env.get_variable(&param_name) {
                let new_name = format!("{}_", overlapping_outside_variable);
                cloned_env.rename_variable(&param_name, new_name)?;
            }

            cloned_env.define_variable(&param_name, super::eval_expr(&cloned_env, db, param)?)?;
        }

        // with the cloned env to evaluate the function params, evaluate the body and return it
        super::eval_code_block(&cloned_env, &self.body_db, self.body.stmts.clone())
    }
}
