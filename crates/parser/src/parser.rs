use crate::{error::ParseError, source::Source};
use ast::*;
use lexer::{Token, TokenKind};
use miette::Result;

pub struct Parser<'input> {
    source: Source<'input>,
}

impl<'input> Parser<'input> {
    pub fn new(source: Source<'input>) -> Self {
        Self { source }
    }

    fn bump(&mut self) -> Result<&Token, ParseError> {
        let eof = self.source.eof();
        let eof_err = Err(ParseError::UnexpectedEof((&self.source).into(), eof));

        match self.source.next_token() {
            Some(t) => Ok(t),
            None => eof_err,
        }
    }

    fn at(&mut self, kind: TokenKind) -> bool {
        self.source.peek_kind() == Some(kind)
    }

    fn at_set(&mut self, set: &[TokenKind]) -> bool {
        self.source.peek_kind().map_or(false, |k| set.contains(&k))
    }

    pub(crate) fn expr(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.at_set(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let op = self.bump()?.kind.into();
            let rhs = self.comparison()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.at_set(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let op = self.bump()?.kind.into();
            let rhs = self.term()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.at_set(&[TokenKind::Minus, TokenKind::Plus]) {
            let op = self.bump()?.kind.into();
            let rhs = self.factor()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.at_set(&[TokenKind::Slash, TokenKind::Star]) {
            let op = self.bump()?.kind.into();
            let rhs = self.unary()?;

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.at_set(&[TokenKind::Bang, TokenKind::Minus]) {
            let op = self.bump()?.kind.into();
            let rhs = self.unary()?;

            Ok(Expr::Unary {
                op,
                rhs: Box::new(rhs),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.at(TokenKind::False) {
            Ok(Expr::Literal(Literal::False))
        } else if self.at(TokenKind::True) {
            Ok(Expr::Literal(Literal::True))
        } else if self.at(TokenKind::Null) {
            Ok(Expr::Literal(Literal::Null))
        } else if self.at(TokenKind::LeftParen) {
            self.bump();
            let expr = self.expr()?;
            let t = self.bump()?;
            if t.kind != TokenKind::RightParen {
                let actual = t.text.to_string();
                let span = t.span;
                Err(ParseError::ExpectedToken(
                    (&self.source).into(),
                    actual,
                    TokenKind::RightParen,
                    span,
                ))
            } else {
                Ok(Expr::Grouping(Box::new(expr)))
            }
        } else if self.at(TokenKind::Number) {
            let token = self.bump()?;
            Ok(Expr::Literal(Literal::Number {
                val: token
                    .text
                    .parse::<f64>()
                    .expect("ICE: Couldn't parse number"),
            }))
        } else if self.at(TokenKind::String) {
            let token = self.bump()?;
            Ok(Expr::Literal(Literal::String {
                val: token.text.into(),
            }))
        } else {
            Err(ParseError::ExpectedExpression(
                (&self.source).into(),
                self.bump()?.span,
            ))
        }
    }
}
