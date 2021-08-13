use syntax::{SyntaxKind, SyntaxNode};

mod expr;
mod stmt;

pub use expr::*;
pub use stmt::*;

#[derive(Debug, Clone)]
pub struct Root(SyntaxNode);

impl Root {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Root {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn stmts(&self) -> impl Iterator<Item = stmt::Stmt> {
        self.0.children().filter_map(stmt::Stmt::cast)
    }
}
