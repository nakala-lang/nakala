use crate::expr::{CodeBlock, Expr};
use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug)]
pub enum Stmt {
    VariableDef(VariableDef),
    Expr(Expr),
    FunctionDef(FunctionDef),
    VariableAssign(VariableAssign),
    If(If),
    ElseIf(ElseIf),
    Else(Else),
    Return(Return),
}

impl Stmt {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::VariableDef => Self::VariableDef(VariableDef(node)),
            SyntaxKind::FunctionDef => Self::FunctionDef(FunctionDef(node)),
            SyntaxKind::VariableAssign => Self::VariableAssign(VariableAssign(node)),
            SyntaxKind::If => Self::If(If(node)),
            SyntaxKind::ElseIf => Self::ElseIf(ElseIf(node)),
            SyntaxKind::Else => Self::Else(Else(node)),
            SyntaxKind::Return => Self::Return(Return(node)),
            _ => Self::Expr(Expr::cast(node)?),
        };

        Some(result)
    }
}

#[derive(Debug)]
pub struct VariableDef(SyntaxNode);

impl VariableDef {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn value(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

#[derive(Debug)]
pub struct FunctionDef(SyntaxNode);

impl FunctionDef {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn param_ident_list(&self) -> Vec<SyntaxToken> {
        self.0
            .children()
            .find(|node| node.kind() == SyntaxKind::ParamIdentList)
            .map_or(vec![], |n| {
                n.children_with_tokens()
                    .filter_map(SyntaxElement::into_token)
                    .filter(|token| token.kind() == SyntaxKind::Ident)
                    .collect()
            })
    }

    pub fn body(&self) -> Option<CodeBlock> {
        self.0
            .children()
            .find(|t| t.kind() == SyntaxKind::CodeBlock)
            .map(CodeBlock)
    }
}
#[derive(Debug)]
pub struct VariableAssign(SyntaxNode);

impl VariableAssign {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0.first_token()
    }

    pub fn value(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

#[derive(Debug)]
pub struct If(SyntaxNode);

impl If {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn body(&self) -> Option<CodeBlock> {
        self.0
            .children()
            .find(|t| t.kind() == SyntaxKind::CodeBlock)
            .map(CodeBlock)
    }

    pub fn else_branch(&self) -> Option<ElseBranch> {
        let else_node = self
            .0
            .children()
            .find(|t| t.kind() == SyntaxKind::Else)
            .map(Else);

        let else_if_node = self
            .0
            .children()
            .find(|t| t.kind() == SyntaxKind::ElseIf)
            .map(ElseIf);

        if let Some(else_stmt) = else_node {
            Some(ElseBranch::Else(else_stmt))
        } else {
            else_if_node.map(ElseBranch::ElseIf)
        }
    }
}

#[derive(Debug)]
pub struct ElseIf(SyntaxNode);

impl ElseIf {
    pub fn if_stmt(&self) -> Option<If> {
        self.0
            .children()
            .find(|t| t.kind() == SyntaxKind::If)
            .map(If)
    }
}

#[derive(Debug)]
pub struct Else(SyntaxNode);

impl Else {
    pub fn body(&self) -> Option<crate::expr::CodeBlock> {
        self.0
            .children()
            .find(|t| t.kind() == SyntaxKind::CodeBlock)
            .map(crate::expr::CodeBlock)
    }
}

#[derive(Debug)]
pub enum ElseBranch {
    Else(Else),
    ElseIf(ElseIf),
}

#[derive(Debug)]
pub struct Return(SyntaxNode);

impl Return {
    pub fn value(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}
