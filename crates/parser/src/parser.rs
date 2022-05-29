use std::collections::HashMap;

use crate::{
    error::ParseError,
    source::Source,
    symtab::{Sym, Symbol, SymbolTable},
    type_check::{result_type, type_compatible},
    Parse,
};
use ast::{
    expr::{Expr, Expression},
    op::{Op, Operator},
    stmt::{Binding, Class, Function, Statement, Stmt},
    ty::{Type, TypeExpression},
};
use lexer::{Token, TokenKind};
use meta::{Span, Spanned};

pub struct Parser {
    source: Source,
    symtab: SymbolTable,
}

impl Parser {
    pub fn new(source: Source, symtab: Option<SymbolTable>) -> Self {
        Self {
            source,
            symtab: symtab.unwrap_or(SymbolTable::new()),
        }
    }

    pub fn parse(mut self) -> miette::Result<Parse> {
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
        if self.at(TokenKind::Class) {
            self.class_decl()
        } else if self.at(TokenKind::Func) {
            self.func_decl(false)
        } else if self.at(TokenKind::Let) {
            self.var_decl()
        } else {
            self.stmt()
        }
    }

    fn class_decl(&mut self) -> Result<Statement, ParseError> {
        let class_token_span = self.expect(TokenKind::Class)?.span;
        let name_token = self.expect(TokenKind::Ident)?;

        let name = name_token.text.to_string();
        let name_span = name_token.span;

        self.expect(TokenKind::LeftBrace)?;

        let mut methods = Vec::new();
        let mut method_symbols = HashMap::default();

        while !self.source.at_end() && !self.at(TokenKind::RightBrace) {
            let stmt = self.func_decl(true)?;
            match stmt.stmt {
                Stmt::Function(func) => {
                    methods.push(func.clone());
                    method_symbols.insert(
                        func.name.item.clone(),
                        Symbol {
                            name: func.name.item.clone(),
                            sym: Sym::Function {
                                arity: func.params.len(),
                            },
                            ty: func.return_ty.ty,
                        },
                    );
                }
                _ => panic!("ICE: func_decl returned a stmt that wasnt a function"),
            }
        }

        self.symtab.insert(Symbol {
            name: name.clone(),
            ty: Type::Class(name.clone()),
            sym: Sym::Class {
                methods: method_symbols,
            },
        });

        let right_brace = self.expect(TokenKind::RightBrace)?;

        Ok(Statement {
            span: Span::combine(&[class_token_span, right_brace.span]),
            stmt: Stmt::Class(Class {
                name: Spanned {
                    item: name,
                    span: name_span,
                },
                methods,
            }),
        })
    }

    fn func_decl(&mut self, from_class_decl: bool) -> Result<Statement, ParseError> {
        let mut start_span: Span = Span::garbage();
        if !from_class_decl {
            start_span = self.expect(TokenKind::Func)?.span;
        }

        let name_token = self.expect(TokenKind::Ident)?;
        let spanned_name = Spanned {
            item: name_token.text.to_string(),
            span: name_token.span
        };
        let name = name_token.text.to_string();

        if from_class_decl {
            start_span = name_token.span;
        }

        self.expect(TokenKind::LeftParen)?;

        let mut params = Vec::new();
        if !self.at(TokenKind::RightParen) {
            loop {
                let param = self.binding()?;
                params.push(param);

                if self.at(TokenKind::Comma) {
                    self.bump()?;
                } else {
                    break;
                }
            }
        }

        let right_paren_span = self.expect(TokenKind::RightParen)?.span;

        // Check if type defined
        let mut return_ty = TypeExpression {
            span: Span::new(0, 0),
            ty: Type::Any,
        };

        if self.at(TokenKind::Arrow) {
            self.bump()?;
            return_ty = self.ty()?;
        }

        if !from_class_decl {
            self.symtab.insert(Symbol {
                name: name.clone(),
                sym: Sym::Function {
                    arity: params.len(),
                },
                ty: return_ty.ty.clone(),
            });

            self.symtab.level_up();

            params.iter().for_each(|param| {
                self.symtab.insert(Symbol {
                    name: param.name.item.clone(),
                    sym: Sym::Variable,
                    ty: param.ty.clone(),
                })
            });
        }

        let body = self.block(true)?;

        if !from_class_decl {
            self.symtab.level_down();
        }

        if let Stmt::Block(stmts) = &body.stmt {
            // If the body has a return statement in it, make sure the types line up
            if let Some(Statement {
                stmt: Stmt::Return(ret),
                ..
            }) = stmts.last()
            {
                if let Some(ret_expr) = ret {
                    if !type_compatible(&ret_expr.ty, &return_ty.ty) {
                        return Err(ParseError::IncompatibleTypes(
                            (&self.source).into(),
                            return_ty.span.into(),
                            return_ty.ty,
                            ret_expr.span.into(),
                            ret_expr.ty.clone(),
                        ));
                    } else {
                        // Update type to the type of the return stmt
                        if !from_class_decl {
                            let sym = self
                                .symtab
                                .lookup_mut(&name)
                                .expect("ICE: couldn't find func symbol to update ret type");
                            sym.ty = ret_expr.ty.clone();
                        }
                        return_ty.ty = ret_expr.ty.clone();
                    }
                }
            } else {
                // If the body has no return statement, make sure there is no return type
                // annotation on the function
                if return_ty.ty != Type::Any {
                    return Err(ParseError::IncompatibleTypes(
                        (&self.source).into(),
                        return_ty.span.into(),
                        return_ty.ty,
                        body.span.into(),
                        Type::Null,
                    ));
                }
            }
        }

        Ok(Statement {
            stmt: Stmt::Function(Function {
                name: spanned_name,
                params,
                body: Box::new(body),
                return_ty,
            }),
            span: Span::combine(&[start_span, right_paren_span]),
        })
    }

    fn var_decl(&mut self) -> Result<Statement, ParseError> {
        let let_token_span = self.bump()?.span;

        let binding = self.binding()?;

        let mut ty = binding.ty.clone();
        let mut expr = None;
        if self.at(TokenKind::Equal) {
            self.bump()?;
            let val = self.expr()?;
            if !type_compatible(&ty, &val.ty) {
                return Err(ParseError::IncompatibleTypes(
                    (&self.source).into(),
                    binding.name.span.into(),
                    binding.ty,
                    val.span.into(),
                    val.ty,
                ));
            }

            ty = val.ty.clone();
            expr = Some(val);
        }

        self.symtab.insert(Symbol {
            sym: Sym::Variable,
            name: binding.name.item.clone(),
            ty,
        });

        let semi_token = self.expect(TokenKind::Semicolon)?;
        Ok(Statement {
            span: Span::combine(&[let_token_span, semi_token.span]),
            stmt: Stmt::Variable {
                name: binding,
                expr,
            },
        })
    }

    fn stmt(&mut self) -> Result<Statement, ParseError> {
        if self.at(TokenKind::Print) {
            self.print_stmt()
        } else if self.at(TokenKind::LeftBrace) {
            self.block(false)
        } else if self.at(TokenKind::If) {
            self.if_stmt()
        } else if self.at(TokenKind::Until) {
            self.until_stmt()
        } else if self.at(TokenKind::Ret) {
            self.ret_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn ret_stmt(&mut self) -> Result<Statement, ParseError> {
        let ret_span = self.expect(TokenKind::Ret)?.span;

        let mut expr: Option<Expression> = None;
        if !self.at(TokenKind::Semicolon) {
            expr = Some(self.expr()?);
        }

        let semi_colon_span = self.bump()?.span;

        Ok(Statement {
            stmt: Stmt::Return(expr),
            span: Span::combine(&[ret_span, semi_colon_span]),
        })
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

    fn block(&mut self, from_func_decl: bool) -> Result<Statement, ParseError> {
        let left_brace_span = self.expect(TokenKind::LeftBrace)?.span;

        if !from_func_decl {
            self.symtab.level_up();
        }

        let mut stmts = Vec::new();
        while !self.source.at_end() && !self.at(TokenKind::RightBrace) {
            stmts.push(self.decl()?);
        }

        let right_brace_span = self.expect(TokenKind::RightBrace)?.span;

        if !from_func_decl {
            self.symtab.level_down();
        }

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
                                ty: rhs.ty.clone(),
                                expr: Expr::Assign {
                                    name: Spanned {
                                        item: name,
                                        span: expr.span,
                                    },
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
                Expr::Get { object, name } => Ok(Expression {
                    span: Span::combine(&[expr.span, rhs.span]),
                    expr: Expr::Set {
                        object,
                        name,
                        rhs: Box::new(rhs),
                    },
                    ty: Type::Null,
                }),
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
                ty: rhs.ty.clone(),
                expr: Expr::Unary {
                    op,
                    rhs: Box::new(rhs),
                },
            })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.at(TokenKind::LeftParen) {
                self.bump()?;
                expr = self.finish_call(expr)?;
            } else if self.at(TokenKind::Dot) {
                self.bump()?;
                let name = self.expect(TokenKind::Ident)?;

                // Can only use dot operator when left hand side is of type Instance
                if !matches!(expr.ty, Type::Any | Type::Instance(..)) {
                    return Err(ParseError::OnlyInstancesHaveProperties(
                        (&self.source).into(),
                        expr.span.into(),
                        expr.ty,
                    ));
                }

                expr = Expression {
                    span: Span::combine(&[expr.span, name.span]),
                    ty: Type::Any,
                    expr: Expr::Get {
                        name: name.into(),
                        object: Box::new(expr),
                    },
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, ParseError> {
        let mut args: Vec<Expression> = Vec::new();

        // Check if we have args
        if !self.at(TokenKind::RightParen) {
            loop {
                args.push(self.expr()?);

                if self.at(TokenKind::Comma) {
                    self.bump()?;
                } else {
                    break;
                }
            }
        }

        let paren = self.expect(TokenKind::RightParen)?.span;

        if !matches!(callee.expr, Expr::Variable(..)) {
            return Err(ParseError::UncallableExpression(
                (&self.source).into(),
                callee.span.into(),
            ));
        }

        let mut ty = callee.ty.clone();
        if let Type::Class(class_name) = ty {
            // It's now an instance
            ty = Type::Instance(class_name);
        }

        Ok(Expression {
            span: Span::combine(&[callee.span.clone(), paren]),
            ty,
            expr: Expr::Call {
                callee: Box::new(callee),
                paren,
                args,
            },
        })
    }

    fn binding(&mut self) -> Result<Binding, ParseError> {
        let ident = self.expect(TokenKind::Ident)?;

        let name = ident.text.to_string();
        let span = ident.span;

        let mut ty = Type::Any;
        if self.at(TokenKind::Colon) {
            self.bump()?;

            ty = self.ty()?.ty;
        }

        Ok(Binding {
            name: Spanned {
                item: name.clone(),
                span,
            },
            ty,
        })
    }

    fn ty(&mut self) -> Result<TypeExpression, ParseError> {
        let token = self.bump()?;
        let span = token.span;

        let ty = match token.kind {
            TokenKind::TypeInt => Type::Int,
            TokenKind::TypeFloat => Type::Float,
            TokenKind::TypeBool => Type::Bool,
            TokenKind::TypeString => Type::String,
            TokenKind::Null => Type::Null,
            TokenKind::TypeAny => Type::Any,
            _ => return Err(ParseError::UnknownType((&self.source).into(), span.into())),
        };

        Ok(TypeExpression { ty, span })
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        let token = self.bump()?;
        let token_span = token.span;

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
                    ty: expr.ty.clone(),
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
                        ty: symbol.ty.clone(),
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
                token_span.into(),
            )),
        }
    }
}
