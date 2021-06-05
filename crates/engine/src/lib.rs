use hir::{Database, Stmt};

pub mod val;
use val::Val;

pub fn eval(hir: (Database, Vec<Stmt>)) -> Val {
    todo!();
}

fn eval_stmt(stmt: Stmt) -> Val {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn eval_add() {}
}
