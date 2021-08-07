use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    Whitespace,
    FnKw,
    CallKw,
    LetKw,
    IfKw,
    ElseKw,
    Ident,
    Number,
    String,
    Boolean,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LParen,
    RParen,
    LBrace,
    RBrace,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    AndKw,
    OrKw,
    NotKw,
    ComparisonEquals,
    Comma,
    Comment,
    Error,
    Root,
    InfixExpr,
    Literal,
    ParenExpr,
    PrefixExpr,
    VariableDef,
    VariableRef,
    VariableAssign,
    CodeBlock,
    FunctionDef,
    FunctionCall,
    ParamIdentList,
    ParamValueList,
    If,
    Else,
    ElseIf,
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::FnKw => Self::FnKw,
            TokenKind::CallKw => Self::CallKw,
            TokenKind::LetKw => Self::LetKw,
            TokenKind::IfKw => Self::IfKw,
            TokenKind::ElseKw => Self::ElseKw,
            TokenKind::Ident => Self::Ident,
            TokenKind::Number => Self::Number,
            TokenKind::String => Self::String,
            TokenKind::Boolean => Self::Boolean,
            TokenKind::Dot => Self::Dot,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Star,
            TokenKind::Slash => Self::Slash,
            TokenKind::Equals => Self::Equals,
            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::GreaterThan => Self::GreaterThan,
            TokenKind::GreaterThanOrEqual => Self::GreaterThanOrEqual,
            TokenKind::LessThan => Self::LessThan,
            TokenKind::LessThanOrEqual => Self::LessThanOrEqual,
            TokenKind::AndKw => Self::AndKw,
            TokenKind::OrKw => Self::OrKw,
            TokenKind::NotKw => Self::NotKw,
            TokenKind::ComparisonEquals => Self::ComparisonEquals,
            TokenKind::Comma => Self::Comma,
            TokenKind::Comment => Self::Comment,
            TokenKind::Error => Self::Error,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NakalaLanguage {}

impl rowan::Language for NakalaLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<NakalaLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<NakalaLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<NakalaLanguage>;
