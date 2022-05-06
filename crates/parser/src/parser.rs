use crate::source::Source;
use lexer::{TokenKind, Token};

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

pub struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source
        }
    }

    fn bump(&mut self) -> &Token {
        self.source.next_token().unwrap()
    }

    fn at(&mut self, kind: TokenKind) -> bool {
        self.source.peek_kind() == Some(kind)
    }

    fn at_set(&mut self, set: &[TokenKind]) -> bool {
        self.source.peek_kind().map_or(false, |k| set.contains(&k))
    }

    pub(crate) fn expr(&mut self) -> Expr {
        self.equality()
    }
    
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.at_set(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let op = self.bump().kind.into();
            let rhs = self.comparison();

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs)
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        
        while self.at_set(&[TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Less, TokenKind::LessEqual]) {
            let op = self.bump().kind.into();
            let rhs = self.term();

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs)
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.at_set(&[TokenKind::Minus, TokenKind::Plus]) {
            let op = self.bump().kind.into();
            let rhs = self.factor();

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs)
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.at_set(&[TokenKind::Slash, TokenKind::Star]) {
            let op = self.bump().kind.into();
            let rhs = self.unary();

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs)
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.at_set(&[TokenKind::Bang, TokenKind::Minus]) {
            let op = self.bump().kind.into();
            let rhs = self.unary();

            Expr::Unary {
                op,
                rhs: Box::new(rhs)
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.at(TokenKind::False) {
            Expr::Literal(Literal::False)
        } else if self.at(TokenKind::True) {
            Expr::Literal(Literal::True)
        } else if self.at(TokenKind::Null) {
            Expr::Literal(Literal::Null)
        } else if self.at(TokenKind::LeftParen) {
            let expr = self.expr();
            let t = self.bump();
            if t.kind != TokenKind::RightParen {
                todo!("error: expected right paren")
            } else {
                Expr::Grouping(Box::new(expr))
            }
        } else if self.at(TokenKind::Number) {
            let token = self.bump();
            Expr::Literal(Literal::Number { val: token.text.parse::<f64>().expect("ICE: Couldn't parse number") })
        } else if self.at(TokenKind::String) {
            let token = self.bump();
            Expr::Literal(Literal::String { val: token.text.into() })
        } else {
            unreachable!("ICE: primary didn't expand")
        }
    }
}
