use crate::env::Env;
use crate::{error::EngineError, val::Val};
use hir::{CodeBlock, Database, Expr, FunctionDef};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    name: String,
    param_list: Vec<String>,
    body: CodeBlock,
}

impl Function {
    pub fn new(func_def: FunctionDef) -> Self {
        Function {
            name: func_def.name.to_string(),
            param_list: func_def
                .param_ident_list
                .into_iter()
                .map(|p| p.to_string())
                .collect(),
            body: func_def.body,
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
            let mut param_name = self.param_list.get(index).unwrap().to_string();

            // Append a `_` every time
            while cloned_env.get_variable(&param_name).is_ok() {
                param_name = format!("{}_", param_name.clone());
            }

            cloned_env.set_variable(&param_name, super::eval_expr(&cloned_env, db, param)?)?;
        }

        // with the cloned env to evaluate the function params, evaluate the body and return it
        super::eval_code_block(&cloned_env, db, self.body.stmts.clone())
    }
}
