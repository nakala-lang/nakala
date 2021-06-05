use hir::{Database, Stmt};

pub mod val;

pub fn eval(hir: (Database, Vec<Stmt>)) -> val::Val {
    todo!()
}
