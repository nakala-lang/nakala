use lexer::TokenKind;

#[derive(Debug, PartialEq)]
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
    Div
}

impl From<TokenKind> for Op {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Equal => Op::Equals,
            TokenKind::BangEqual => Op::NotEquals,
            TokenKind::Less => Op::LessThan,
            TokenKind::LessEqual => Op::LessThanEquals,
            TokenKind::Greater => Op::GreaterThan,
            TokenKind::GreaterEqual => Op::GreaterThanEquals,
            TokenKind::Plus => Op::Add,
            TokenKind::Minus => Op::Sub,
            TokenKind::Star => Op::Mul,
            TokenKind::Slash => Op::Div,
            _ => unreachable!("ICE : Tried to convert non-op token into Op enum")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Unary {
        op: Op,
        rhs: Box<Expr>
    },
    Binary {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>
    },
    Grouping(Box<Expr>)
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number {
        val: f64
    },
    String {
        val: String
    },
    True,
    False,
    Null
}
