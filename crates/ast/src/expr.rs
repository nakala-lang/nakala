use crate::stmt::Stmt;
use itertools::Itertools;
use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    Literal(Literal),
    ParenExpr(ParenExpr),
    UnaryExpr(UnaryExpr),
    VariableRef(VariableRef),
    CodeBlock(CodeBlock),
    FunctionCall(FunctionCall),
    List(List),
    IndexOp(IndexOp),
    ClassCreate(ClassCreate),
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
            SyntaxKind::FunctionCall => Self::FunctionCall(FunctionCall(node)),
            SyntaxKind::List => Self::List(List(node)),
            SyntaxKind::IndexOp => Self::IndexOp(IndexOp(node)),
            SyntaxKind::ClassCreate => Self::ClassCreate(ClassCreate(node)),
            _ => {
                return None;
            }
        };

        Some(result)
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
                    SyntaxKind::Plus
                        | SyntaxKind::Minus
                        | SyntaxKind::Star
                        | SyntaxKind::Slash
                        | SyntaxKind::GreaterThan
                        | SyntaxKind::GreaterThanOrEqual
                        | SyntaxKind::LessThan
                        | SyntaxKind::LessThanOrEqual
                        | SyntaxKind::AndKw
                        | SyntaxKind::OrKw
                        | SyntaxKind::ComparisonEquals
                )
            })
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(Number),
    String(self::String),
    Boolean(Boolean),
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
            SyntaxKind::Boolean => Self::Boolean(Boolean(node)),
            _ => unreachable!("Literals must be Numbers or Strings"),
        }
    }
}

#[derive(Debug)]
pub struct Number(SyntaxNode);

impl Number {
    pub fn parse(&self) -> f64 {
        self.0
            .children_with_tokens()
            .filter(|t| t.kind() == SyntaxKind::Number)
            .map(|t| t.to_string())
            .join(".")
            .parse()
            .unwrap()
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
pub struct Boolean(SyntaxNode);

impl Boolean {
    pub fn parse(&self) -> bool {
        self.0.first_token().unwrap().text().parse().unwrap()
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
            .find(|token| matches!(token.kind(), SyntaxKind::Minus | SyntaxKind::NotKw))
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
pub struct CodeBlock(pub(crate) SyntaxNode);

impl CodeBlock {
    pub fn stmts(&self) -> Vec<Stmt> {
        self.0.children().filter_map(Stmt::cast).collect()
    }
}

#[derive(Debug)]
pub struct FunctionCall(SyntaxNode);

impl FunctionCall {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn param_value_list(&self) -> Vec<Expr> {
        self.0
            .children()
            .find(|node| node.kind() == SyntaxKind::ParamValueList)
            .map_or(vec![], |n| n.children().filter_map(Expr::cast).collect())
    }
}

#[derive(Debug)]
pub struct List(SyntaxNode);

impl List {
    pub fn items(&self) -> Vec<Expr> {
        self.0.children().filter_map(Expr::cast).collect()
    }
}

#[derive(Debug)]
pub struct IndexOp(SyntaxNode);

impl IndexOp {
    pub fn ident(&self) -> Option<SyntaxToken> {
        self.0.first_token()
    }

    pub fn index(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

#[derive(Debug)]
pub struct ClassCreate(SyntaxNode);

impl ClassCreate {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn param_value_list(&self) -> Vec<Expr> {
        self.0
            .children()
            .find(|node| node.kind() == SyntaxKind::ParamValueList)
            .map_or(vec![], |n| n.children().filter_map(Expr::cast).collect())
    }
}
