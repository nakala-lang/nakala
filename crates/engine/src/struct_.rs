use crate::{func::Function, val::Val};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Struct {
    pub name: String,
    pub members: HashMap<String, StructMember>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StructMember {
    Val(Val),
    Function(Function),
}
