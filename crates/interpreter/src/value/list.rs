use super::Indexible;
use crate::{env::Environment, error::RuntimeError, Val, Value};
use ast::ty::Type;
use meta::Span;

pub type ListId = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    id: ListId,
    values: Vec<Value>,
}

impl List {
    pub fn new(id: ListId, values: Vec<Value>) -> Self {
        Self { id, values }
    }

    pub fn to_string(&self, env: &mut Environment) -> String {
        format!(
            "[{}]",
            self.values
                .clone()
                .into_iter()
                .map(|val| val.to_string(env))
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    pub fn len(&self) -> Value {
        Value {
            val: Val::Int(self.values.len() as i64),
            span: Span::garbage(),
            ty: Type::Int,
        }
    }
}

impl Indexible for List {
    fn get(&self, index: usize) -> Result<Value, RuntimeError> {
        if let Some(val) = self.values.get(index) {
            Ok(val.clone())
        } else {
            todo!("not found");
        }
    }

    fn set(&mut self, index: usize, val: Value) -> Result<(), RuntimeError> {
        if index >= self.values.len() {
            todo!("out of bounds");
        } else {
            self.values[index] = val;
            Ok(())
        }
    }
}
