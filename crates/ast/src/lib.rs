use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

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
            .filter(|node| node.kind() == SyntaxKind::ParamIdentList)
            .nth(0)
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
            .map_or(None, |v| Some(CodeBlock(v)))
    }
}

#[derive(Debug)]
pub struct BinaryExpr(SyntaxNode);

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children().filter_map(Expr::cast).nth(1)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| {
                matches!(
                    token.kind(),
                    SyntaxKind::Plus | SyntaxKind::Minus | SyntaxKind::Star | SyntaxKind::Slash
                )
            })
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(Number),
    String(crate::String),
}

impl Literal {
    pub fn cast(node: SyntaxNode) -> Self {
        match node
            .first_token()
            .expect("Literal's must have a child")
            .kind()
        {
            SyntaxKind::Number => Self::Number(Number(node)),
            SyntaxKind::String => Self::String(String(node)),
            _ => unreachable!("Literals must be Numbers or Strings"),
        }
    }
}

#[derive(Debug)]
pub struct Number(SyntaxNode);

impl Number {
    pub fn parse(&self) -> u64 {
        self.0.first_token().unwrap().text().parse().unwrap()
    }
}

#[derive(Debug)]
pub struct String(SyntaxNode);

impl String {
    pub fn parse(&self) -> std::string::String {
        self.0.first_token().unwrap().text().to_string()
    }
}

#[derive(Debug)]
pub struct ParenExpr(SyntaxNode);

impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

#[derive(Debug)]
pub struct UnaryExpr(SyntaxNode);

impl UnaryExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Minus)
    }
}

#[derive(Debug)]
pub struct VariableRef(SyntaxNode);

impl VariableRef {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0.first_token()
    }
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    Literal(Literal),
    ParenExpr(ParenExpr),
    UnaryExpr(UnaryExpr),
    VariableRef(VariableRef),
    CodeBlock(CodeBlock),
}

impl Expr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::InfixExpr => Self::BinaryExpr(BinaryExpr(node)),
            SyntaxKind::Literal => Self::Literal(Literal::cast(node)),
            SyntaxKind::ParenExpr => Self::ParenExpr(ParenExpr(node)),
            SyntaxKind::PrefixExpr => Self::UnaryExpr(UnaryExpr(node)),
            SyntaxKind::VariableRef => Self::VariableRef(VariableRef(node)),
            SyntaxKind::CodeBlock => Self::CodeBlock(CodeBlock(node)),
            _ => {
                return None;
            }
        };

        Some(result)
    }
}

#[derive(Debug)]
pub struct CodeBlock(SyntaxNode);

impl CodeBlock {
    pub fn stmts(&self) -> Vec<Stmt> {
        self.0.children().filter_map(Stmt::cast).collect()
    }
}

#[derive(Debug)]
pub enum Stmt {
    VariableDef(VariableDef),
    Expr(Expr),
    FunctionDef(FunctionDef),
}

impl Stmt {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::VariableDef => Self::VariableDef(VariableDef(node)),
            SyntaxKind::FunctionDef => Self::FunctionDef(FunctionDef(node)),
            _ => Self::Expr(Expr::cast(node)?),
        };

        Some(result)
    }
}

#[derive(Debug)]
pub struct Root(SyntaxNode);

impl Root {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Root {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn stmts(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast)
    }
}
