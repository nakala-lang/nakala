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
    Div,
    And,
    Or
}

impl From<TokenKind> for Op {
    fn from(kind: TokenKind) -> Self {
        match kind {
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
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Unary {
        op: Op,
        rhs: Box<Expr>,
    },
    Binary {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Variable(String),
    Assign {
        name: String,
        rhs: Box<Expr>
    },
    // Logical expressions short circuit, unlike Binary
    Logical {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Null,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Variable { name: String, expr: Option<Expr> },
    Block(Vec<Stmt>),
    If {
        cond: Expr,
        body: Box<Stmt>,
        else_branch: Option<Box<Stmt>>
    },
    Until {
        cond: Expr,
        body: Box<Stmt>
    }
}
