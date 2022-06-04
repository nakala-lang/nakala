use std::collections::HashMap;

use ast::stmt::Class as AstClass;

use super::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub class: AstClass,
    pub methods: HashMap<String, Value>,
}
