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

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        let t = self.bump()?;
        if t.kind != kind {
            let actual = t.text.to_string();
            let span = t.span;
            Err(ParseError::ExpectedToken(
                (&self.source).into(),
                actual,
                kind,
                span,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn program(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.source.at_end() {
            stmts.push(self.stmt()?);
        }

        Ok(stmts)
    }

    fn stmt(&mut self) -> Result<Stmt, ParseError> {
        if self.at(TokenKind::Print) {
            self.bump()?;
            self.print_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Print(expr))
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Expr(expr))
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
            self.bump()?;
            Ok(Expr::Literal(Literal::False))
        } else if self.at(TokenKind::True) {
            self.bump()?;
            Ok(Expr::Literal(Literal::True))
        } else if self.at(TokenKind::Null) {
            self.bump()?;
            Ok(Expr::Literal(Literal::Null))
        } else if self.at(TokenKind::LeftParen) {
            // skip over LeftParen
            let _ = self.bump();

            let expr = self.expr()?;
            self.expect(TokenKind::RightParen)?;
            Ok(Expr::Grouping(Box::new(expr)))
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

            // Trim the first and last char, as they are " characters
            let mut token_text = token.text.to_string();
            token_text.remove(0);
            token_text.remove(token_text.len() - 1);

            Ok(Expr::Literal(Literal::String {
                val: token_text
            }))
        } else {
            Err(ParseError::ExpectedExpression(
                (&self.source).into(),
                self.bump()?.span,
            ))
        }
    }
}
