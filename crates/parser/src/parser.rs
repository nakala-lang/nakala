use crate::{
    error::ParseError,
    source::Source,
    symtab::{Symbol, SymbolTable},
    type_check::{result_type, type_compatible},
    Parse,
};
use ast::{
    expr::{Expr, Expression},
    op::{Op, Operator},
    stmt::{Statement, Stmt},
    ty::Type,
};
use lexer::{Token, TokenKind};
use meta::Span;
use miette::Result;

pub struct Parser<'input> {
    source: Source<'input>,
    symtab: SymbolTable,
}

impl<'input> Parser<'input> {
    pub fn new(source: Source<'input>) -> Self {
        Self {
            source,
            symtab: SymbolTable::new(),
        }
    }

    pub fn parse(mut self) -> Result<Parse, ParseError> {
        Ok(Parse {
            stmts: self.program()?,
            symtab: self.symtab,
        })
    }

    fn result_type(
        &self,
        lhs: &Expression,
        op: &Operator,
        rhs: &Expression,
    ) -> Result<Type, ParseError> {
        if let Some(ty) = result_type(lhs, op, rhs) {
            Ok(ty)
        } else {
            Err(ParseError::UnsupportedOperation(
                (&self.source).into(),
                op.span.into(),
                lhs.span.into(),
                lhs.ty.clone(),
                rhs.span.into(),
                rhs.ty.clone(),
            ))
        }
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
            Err(ParseError::ExpectedToken(
                error_source,
                actual,
                kind,
                span.into(),
            ))
        }
    }

    fn program(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut stmts: Vec<Statement> = Vec::new();
        while !self.source.at_end() {
            stmts.push(self.decl()?);
        }

        Ok(stmts)
    }

    fn decl(&mut self) -> Result<Statement, ParseError> {
        if self.at(TokenKind::Let) {
            self.var_decl()
        } else {
            self.stmt()
        }
    }

    fn var_decl(&mut self) -> Result<Statement, ParseError> {
        let let_token_span = self.bump()?.span;

        let ident = self.expect(TokenKind::Ident)?;
        let name = ident.text.to_string();

        let mut ty = Type::Null;
        let mut expr = None;
        if self.at(TokenKind::Equal) {
            self.bump()?;
            let val = self.expr()?;
            ty = val.ty;
            expr = Some(val);
        }

        self.symtab.insert(name.clone(), ty);

        let semi_token = self.expect(TokenKind::Semicolon)?;
        Ok(Statement {
            span: Span::combine(&[let_token_span, semi_token.span]),
            stmt: Stmt::Variable { name, expr },
        })
    }

    fn stmt(&mut self) -> Result<Statement, ParseError> {
        if self.at(TokenKind::Print) {
            self.print_stmt()
        } else if self.at(TokenKind::LeftBrace) {
            self.block()
        } else if self.at(TokenKind::If) {
            self.if_stmt()
        } else if self.at(TokenKind::Until) {
            self.until_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn until_stmt(&mut self) -> Result<Statement, ParseError> {
        let until_token_span = self.expect(TokenKind::Until)?.span;

        self.expect(TokenKind::LeftParen)?;
        let cond = self.expr()?;
        self.expect(TokenKind::RightParen)?;

        let body = self.stmt()?;

        Ok(Statement {
            span: Span::combine(&[until_token_span, body.span]),
            stmt: Stmt::Until {
                cond,
                body: Box::new(body),
            },
        })
    }

    fn if_stmt(&mut self) -> Result<Statement, ParseError> {
        let if_token_span = self.expect(TokenKind::If)?.span;

        self.expect(TokenKind::LeftParen)?;
        let cond = self.expr()?;
        self.expect(TokenKind::RightParen)?;

        let body = self.stmt()?;

        let mut else_branch = None;
        if self.at(TokenKind::Else) {
            self.bump()?;
            else_branch = Some(Box::new(self.stmt()?));
        }

        Ok(Statement {
            span: Span::combine(&[if_token_span, body.span]),
            stmt: Stmt::If {
                cond,
                body: Box::new(body),
                else_branch,
            },
        })
    }

    fn block(&mut self) -> Result<Statement, ParseError> {
        let left_brace_span = self.expect(TokenKind::LeftBrace)?.span;

        self.symtab.level_up();

        let mut stmts = Vec::new();
        while !self.source.at_end() && !self.at(TokenKind::RightBrace) {
            stmts.push(self.decl()?);
        }

        let right_brace_span = self.expect(TokenKind::RightBrace)?.span;

        self.symtab.level_down();

        Ok(Statement {
            stmt: Stmt::Block(stmts),
            span: Span::combine(&[left_brace_span, right_brace_span]),
        })
    }

    fn print_stmt(&mut self) -> Result<Statement, ParseError> {
        let print_token_span = self.expect(TokenKind::Print)?.span;

        let expr = self.expr()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Statement {
            span: Span::combine(&[print_token_span, semi.span]),
            stmt: Stmt::Print(expr),
        })
    }

    fn expr_stmt(&mut self) -> Result<Statement, ParseError> {
        let expr = self.expr()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Statement {
            span: Span::combine(&[expr.span, semi.span]),
            stmt: Stmt::Expr(expr),
        })
    }

    fn expr(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
        let expr = self.or()?;

        if self.at(TokenKind::Equal) {
            let error_source = (&self.source).into();
            let eq_span = self.bump()?.span;

            let rhs = self.assignment()?;

            return match expr.expr {
                Expr::Variable(name) => {
                    if let Some(entry) = self.symtab.lookup_mut(&name) {
                        if type_compatible(&entry.ty, &rhs.ty) {

                            (*entry).ty = rhs.ty.clone();

                            Ok(Expression {
                                ty: rhs.ty,
                                expr: Expr::Assign {
                                    name,
                                    rhs: Box::new(rhs),
                                },
                                span: expr.span,
                            })
                        } else {
                            Err(ParseError::IncompatibleTypes(
                                (&self.source).into(),
                                expr.span.into(),
                                entry.ty.clone(),
                                rhs.span.into(),
                                rhs.ty.clone(),
                            ))
                        }
                    } else {
                        Err(ParseError::UndeclaredVariable(
                            (&self.source).into(),
                            expr.span.into(),
                            name,
                        ))
                    }
                }
                _ => Err(ParseError::InvalidAssignmentTarget(
                    error_source,
                    eq_span.into(),
                )),
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.and()?;

        while self.at(TokenKind::Or) {
            let op: Operator = self.bump()?.into();
            let rhs = self.and()?;

            let ty = self.result_type(&expr, &op, &rhs)?;

            expr = Expression {
                span: Span::combine(&[op.span, rhs.span]),
                ty,
                expr: Expr::Logical {
                    lhs: Box::new(expr),
                    op,
                    rhs: Box::new(rhs),
                },
            }
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;

        while self.at(TokenKind::And) {
            let op: Operator = self.bump()?.into();
            let rhs = self.equality()?;

            let ty = self.result_type(&expr, &op, &rhs)?;

            expr = Expression {
                span: Span::combine(&[op.span, rhs.span]),
                ty,
                expr: Expr::Logical {
                    lhs: Box::new(expr),
                    op,
                    rhs: Box::new(rhs),
                },
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;

        while self.at_set(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let op: Operator = self.bump()?.into();
            let rhs = self.comparison()?;

            let ty = self.result_type(&expr, &op, &rhs)?;

            expr = Expression {
                span: Span::combine(&[op.span, rhs.span]),
                ty,
                expr: Expr::Binary {
                    lhs: Box::new(expr),
                    op,
                    rhs: Box::new(rhs),
                },
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.term()?;

        while self.at_set(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let op: Operator = self.bump()?.into();
            let rhs = self.term()?;

            let ty = self.result_type(&expr, &op, &rhs)?;

            expr = Expression {
                span: Span::combine(&[op.span, rhs.span]),
                ty,
                expr: Expr::Binary {
                    lhs: Box::new(expr),
                    op,
                    rhs: Box::new(rhs),
                },
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.factor()?;

        while self.at_set(&[TokenKind::Minus, TokenKind::Plus]) {
            let op = self.bump()?.into();
            let rhs = self.factor()?;

            let ty = self.result_type(&expr, &op, &rhs)?;

            expr = Expression {
                span: Span::combine(&[expr.span, rhs.span]),
                ty,
                expr: Expr::Binary {
                    lhs: Box::new(expr),
                    op,
                    rhs: Box::new(rhs),
                },
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.unary()?;

        while self.at_set(&[TokenKind::Slash, TokenKind::Star]) {
            let op = self.bump()?.into();
            let rhs = self.unary()?;

            let ty = self.result_type(&expr, &op, &rhs)?;

            expr = Expression {
                span: Span::combine(&[expr.span, rhs.span]),
                expr: Expr::Binary {
                    lhs: Box::new(expr),
                    op,
                    rhs: Box::new(rhs),
                },
                ty,
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        if self.at_set(&[TokenKind::Bang, TokenKind::Minus]) {
            let op: Operator = self.bump()?.into();
            let rhs = self.unary()?;

            if op.op == Op::Not && rhs.ty != Type::Bool {
                return Err(ParseError::UnsupportedUnaryOperation(
                    (&self.source).into(),
                    op.span.into(),
                    rhs.span.into(),
                    rhs.ty.clone(),
                ));
            }

            if op.op == Op::Sub && (rhs.ty != Type::Float && rhs.ty != Type::Int) {
                return Err(ParseError::UnsupportedUnaryOperation(
                    (&self.source).into(),
                    op.span.into(),
                    rhs.span.into(),
                    rhs.ty.clone(),
                ));
            }

            Ok(Expression {
                span: Span::combine(&[op.span, rhs.span]),
                ty: rhs.ty,
                expr: Expr::Unary {
                    op,
                    rhs: Box::new(rhs),
                },
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        let token = self.bump()?;
        match token.kind {
            TokenKind::False => Ok(Expression {
                expr: Expr::Bool(false),
                span: token.span,
                ty: Type::Bool,
            }),
            TokenKind::True => Ok(Expression {
                expr: Expr::Bool(true),
                span: token.span,
                ty: Type::Bool,
            }),
            TokenKind::Null => Ok(Expression {
                expr: Expr::Null,
                span: token.span,
                ty: Type::Null,
            }),
            TokenKind::LeftParen => {
                let span = token.span;
                let expr = self.expr()?;

                let right_paren = self.expect(TokenKind::RightParen)?;
                Ok(Expression {
                    ty: expr.ty,
                    expr: Expr::Grouping(Box::new(expr)),
                    span: Span::combine(&[span, right_paren.span]),
                })
            }
            TokenKind::Ident => {
                let ident = token.text.to_string();
                let span = token.span;
                if let Some(symbol) = self.symtab.lookup(&ident) {
                    Ok(Expression {
                        expr: Expr::Variable(ident),
                        span,
                        ty: symbol.ty,
                    })
                } else {
                    Err(ParseError::UndeclaredVariable(
                        (&self.source).into(),
                        span.into(),
                        ident,
                    ))
                }
            }
            TokenKind::Int => {
                let val = token
                    .text
                    .parse::<i64>()
                    .expect("ICE: Couldn't parse int as int");
                Ok(Expression {
                    expr: Expr::Int(val),
                    span: token.span,
                    ty: Type::Int,
                })
            }
            TokenKind::Float => {
                let val = token
                    .text
                    .parse::<f64>()
                    .expect("ICE: Couldn't parse float as float");
                Ok(Expression {
                    expr: Expr::Float(val),
                    span: token.span,
                    ty: Type::Float,
                })
            }
            TokenKind::String => {
                println!("{:?}", token.text);

                // Trim the first and last char, as they are " characters
                let mut token_text = token.text.to_string();
                token_text.remove(0);
                token_text.remove(token_text.len() - 1);

                Ok(Expression {
                    expr: Expr::String(token_text),
                    span: token.span,
                    ty: Type::String,
                })
            }
            _ => Err(ParseError::ExpectedExpression(
                (&self.source).into(),
                self.bump()?.span.into(),
            )),
        }
    }
}
