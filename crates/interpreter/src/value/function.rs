use ast::stmt::Function as AstFunction;

use crate::env::ScopeId;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub func: AstFunction,
    pub closure: ScopeId,
}


