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

    fn expect(&mut self, kind: TokenKind) -> Result<&Token, ParseError> {
        let error_source = (&self.source).into();

        let t = self.bump()?;
        if t.kind == kind {
            Ok(t)
        } else {
            let actual = t.text.to_string();
            let span = t.span;
            Err(ParseError::ExpectedToken(error_source, actual, kind, span))
        }
    }

    pub(crate) fn program(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.source.at_end() {
            stmts.push(self.decl()?);
        }

        Ok(stmts)
    }

    fn decl(&mut self) -> Result<Stmt, ParseError> {
        if self.at(TokenKind::Let) {
            self.bump()?;
            self.var_decl()
        } else {
            self.stmt()
        }
    }

    fn var_decl(&mut self) -> Result<Stmt, ParseError> {
        let ident = self.expect(TokenKind::Ident)?;
        let name = ident.text.to_string();

        let mut expr = None;
        if self.at(TokenKind::Equal) {
            self.bump()?;
            expr = Some(self.expr()?);
        }

        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Variable { name, expr })
    }

    fn stmt(&mut self) -> Result<Stmt, ParseError> {
        if self.at(TokenKind::Print) {
            self.bump()?;
            self.print_stmt()
        } else if self.at(TokenKind::LeftBrace) {
            self.bump()?;
            self.block()
        } else {
            self.expr_stmt()
        }
    }

    fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut stmts = Vec::new();
     
        while !self.source.at_end() && !self.at(TokenKind::RightBrace) {
            stmts.push(self.decl()?);
        }

        self.expect(TokenKind::RightBrace)?;
        Ok(Stmt::Block(stmts))
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

    fn expr(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.equality()?;

        if self.at(TokenKind::Equal) {
            let error_source = (&self.source).into();
            let eq_span = self.bump()?.span;

            let rhs = self.assignment()?;

            return match expr {
                Expr::Variable(name) => {
                    Ok(Expr::Assign {
                        name,
                        rhs: Box::new(rhs)
                    })
                }
                _ => Err(ParseError::InvalidAssignmentTarget(
                    error_source,
                    eq_span
                )),
            }
        }

        Ok(expr)
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
        } else if self.at(TokenKind::Ident) {
            let name = self.bump()?.text.to_string();
            Ok(Expr::Variable(name))
        } else if self.at(TokenKind::Number) {
            let token = self.bump()?;
            Ok(Expr::Literal(Literal::Number(
                token
                    .text
                    .parse::<f64>()
                    .expect("ICE: Couldn't parse number"),
            )))
        } else if self.at(TokenKind::String) {
            let token = self.bump()?;

            // Trim the first and last char, as they are " characters
            let mut token_text = token.text.to_string();
            token_text.remove(0);
            token_text.remove(token_text.len() - 1);

            Ok(Expr::Literal(Literal::String(token_text)))
        } else {
            Err(ParseError::ExpectedExpression(
                (&self.source).into(),
                self.bump()?.span,
            ))
        }
    }
}
