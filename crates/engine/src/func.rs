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
        env: &mut Env,
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
        let mut function_env = Env::new(Some(Box::new(env.clone())));

        for (index, param) in params.into_iter().enumerate() {
            let param_name = self.param_list.get(index).unwrap().to_string();

            let val = super::eval_expr(&mut function_env, db, param)?;
            function_env.define_variable(&param_name, val.clone())?;
        }

        // with the cloned env to evaluate the function params, evaluate the body and return it
        let result =
            super::eval_code_block(&mut function_env, &self.body_db, self.body.stmts.clone());

        function_env.propagate_enclosing_env_changes(env);

        if let Err(EngineError::EarlyReturn { value }) = result {
            Ok(value)
        } else {
            result
        }
    }
}
