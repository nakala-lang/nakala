use meta::Span;
use lexer::{Token, TokenKind};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Not
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Operator {
    pub op: Op,
    pub span: Span
}

impl<'a> From<&Token<'a>> for Operator {
    fn from(token: &Token) -> Self {
        
        let op = match token.kind {
            TokenKind::Bang => Op::Not,
            TokenKind::EqualEqual => Op::Equals,
            TokenKind::BangEqual => Op::NotEquals,
            TokenKind::Less => Op::LessThan,
            TokenKind::LessEqual => Op::LessThanEquals,
            TokenKind::Greater => Op::GreaterThan,
            TokenKind::GreaterEqual => Op::GreaterThanEquals,
            TokenKind::Plus => Op::Add,
            TokenKind::Minus => Op::Sub,
            TokenKind::Star => Op::Mul,
            TokenKind::Slash => Op::Div,
            TokenKind::And => Op::And,
            TokenKind::Or => Op::Or,
            _ => unreachable!("ICE : Tried to convert non-op token into Op enum"),
        };

        Self {
            op,
            span: token.span
        }
    }
}
