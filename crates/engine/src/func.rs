use hir::{CodeBlock, FunctionDef};

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
}
