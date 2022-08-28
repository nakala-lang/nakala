use std::collections::HashMap;

use crate::{
    error::ParseError,
    source::Source,
    symtab::{Sym, Symbol, SymbolTable},
    Parse,
};
use ast::{
    expr::{Expr, Expression},
    op::{Op, Operator},
    stmt::{Binding, Class, Function, Statement, Stmt},
    ty::{result_type, type_compatible, Type, TypeExpression},
};
use lexer::{Token, TokenKind};
use meta::{trace, Span, Spanned};

pub struct Parser {
    source: Source,
    symtab: SymbolTable,
}

impl Parser {
    pub fn new(source: Source, symtab: SymbolTable) -> Self {
        Self { source, symtab }
    }

    pub fn parse(mut self) -> miette::Result<Parse> {
        Ok(Parse {
            stmts: self.program()?,
            symtab: self.symtab,
        })
    }

    fn is_callable(&self, callee: &Expression) -> Result<(), ParseError> {
        let err = Err(ParseError::UncallableExpression(
            self.source.id,
            callee.span.into(),
            callee.ty.clone(),
        ));

        match &callee.expr {
            Expr::Variable(name) => {
                if let Some(entry) = self.symtab.lookup(name) {
                    if matches!(
                        entry.ty,
                        Type::Any | Type::Function { .. } | Type::Class(..)
                    ) {
                        return Ok(());
                    }
                }

                err
            }

            // TODO nested get exprs
            Expr::Get { .. } => Ok(()),
            _ => err,
        }
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
                self.source.id,
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
        let eof_err = Err(ParseError::UnexpectedEof(self.source.id, eof));

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
        let error_source = self.source.id;

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
        trace!("parse_program");
        let mut stmts: Vec<Statement> = Vec::new();
        while !self.source.at_end() {
            stmts.push(self.decl()?);
        }

        Ok(stmts)
    }

    fn decl(&mut self) -> Result<Statement, ParseError> {
        trace!("parse_decl");
        if self.at(TokenKind::Class) {
            self.class_decl()
        } else if self.at(TokenKind::Func) {
            self.func_decl(false)
        } else if self.at(TokenKind::Let) {
            self.var_decl()
        } else if self.at(TokenKind::Enum) {
            self.enum_decl()
        } else {
            self.stmt()
        }
    }

    // Enums are just syntactical sugar for static classes
    fn enum_decl(&mut self) -> Result<Statement, ParseError> {
        trace!("parse_enum_decl");
        let enum_token_span = self.expect(TokenKind::Enum)?.span;
        let name_token = self.expect(TokenKind::Ident)?;

        let spanned_name: Spanned<String> = name_token.into();

        let name = name_token.text.to_string();
        let name_span = name_token.span;

        self.expect(TokenKind::LeftBrace)?;

        let mut statics = Vec::new();
        let mut static_symbols = HashMap::default();

        if !self.at(TokenKind::RightBrace) {
            loop {
                let enum_kind = self.expect(TokenKind::Ident)?;
                let stmt = Statement {
                    stmt: Stmt::Variable {
                        name: Binding {
                            name: enum_kind.into(),
                            ty: Type::Int,
                        },
                        expr: Some(Expression {
                            expr: Expr::Int(statics.len() as i64),
                            span: Span::garbage(),
                            ty: Type::Int,
                        }),
                    },
                    span: enum_kind.span,
                };

                static_symbols.insert(
                    enum_kind.text.clone(),
                    Symbol {
                        sym: Sym::Variable,
                        name: enum_kind.into(),
                        ty: Type::Int,
                    },
                );

                statics.push(stmt);

                if self.at(TokenKind::Comma) {
                    self.bump()?;
                } else {
                    break;
                }
            }
        }

        self.symtab.insert(Symbol {
            name: spanned_name,
            ty: Type::Class(name.clone()),
            sym: Sym::Class {
                methods: HashMap::default(),
                statics: static_symbols,
            },
        });

        let right_brace = self.expect(TokenKind::RightBrace)?;

        Ok(Statement {
            span: Span::combine(&[enum_token_span, right_brace.span]),
            stmt: Stmt::Class(Class {
                name: Spanned {
                    item: name,
                    span: name_span,
                },
                methods: vec![],
                statics,
            }),
        })
    }

    fn class_decl(&mut self) -> Result<Statement, ParseError> {
        trace!("parse_class_decl");
        let class_token_span = self.expect(TokenKind::Class)?.span;
        let name_token = self.expect(TokenKind::Ident)?;
        let spanned_name: Spanned<String> = name_token.into();

        let name = name_token.text.to_string();
        let name_span = name_token.span;

        self.expect(TokenKind::LeftBrace)?;

        let mut methods = Vec::new();
        let mut method_symbols = HashMap::default();

        let mut statics = Vec::new();
        let mut static_symbols = HashMap::default();

        while !self.source.at_end() && !self.at(TokenKind::RightBrace) {
            if self.at(TokenKind::Static) {
                let static_token_span = self.bump()?.span;
                let binding = self.binding()?;

                let mut ty = binding.ty.clone();
                let mut expr = None;
                if self.at(TokenKind::Equal) {
                    self.bump()?;
                    let val = self.expr()?;
                    if !type_compatible(&ty, &val.ty) {
                        return Err(ParseError::IncompatibleTypes(
                            self.source.id,
                            binding.name.span.into(),
                            binding.ty,
                            val.span.into(),
                            val.ty,
                        ));
                    }

                    ty = val.ty.clone();
                    expr = Some(val);
                }

                static_symbols.insert(
                    binding.name.item.clone(),
                    Symbol {
                        sym: Sym::Variable,
                        name: binding.name.clone(),
                        ty,
                    },
                );

                let semi_token = self.expect(TokenKind::Semicolon)?;
                statics.push(Statement {
                    span: Span::combine(&[static_token_span, semi_token.span]),
                    stmt: Stmt::Variable {
                        name: binding,
                        expr,
                    },
                });
            } else {
                let stmt = self.func_decl(true)?;
                match stmt.clone().stmt {
                    Stmt::Function(func) => {
                        methods.push(stmt);
                        method_symbols.insert(
                            func.name.item.clone(),
                            Symbol {
                                name: func.name.clone(),
                                sym: Sym::Function {
                                    arity: func.params.len(),
                                },
                                ty: func.ty.ty,
                            },
                        );
                    }
                    _ => panic!("ICE: func_decl returned a stmt that wasnt a function"),
                }
            }
        }

        self.symtab.insert(Symbol {
            name: spanned_name,
            ty: Type::Class(name.clone()),
            sym: Sym::Class {
                methods: method_symbols,
                statics: static_symbols,
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
                statics,
            }),
        })
    }

    fn func_decl(&mut self, from_class_decl: bool) -> Result<Statement, ParseError> {
        trace!("parse_func_decl");
        let mut start_span: Span = Span::garbage();
        if !from_class_decl {
            start_span = self.expect(TokenKind::Func)?.span;
        }

        let name_token = self.expect(TokenKind::Ident)?;
        let spanned_name = Spanned {
            item: name_token.text.to_string(),
            span: name_token.span,
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
            span: Span::garbage(),
            ty: Type::Any,
        };

        if self.at(TokenKind::Arrow) {
            self.bump()?;
            return_ty = self.ty()?;
        }

        let func_type = Type::Function {
            params: params
                .clone()
                .into_iter()
                .map(|param| param.into())
                .collect(),
            returns: Box::new(return_ty.clone()),
        };

        if !from_class_decl {
            self.symtab.insert(Symbol {
                name: spanned_name.clone(),
                sym: Sym::Function {
                    arity: params.len(),
                },
                ty: func_type.clone(),
            });
        }

        self.symtab.level_up();

        params.iter().for_each(|param| {
            self.symtab.insert(Symbol {
                name: param.name.clone(),
                sym: Sym::Variable,
                ty: param.ty.clone(),
            })
        });

        let body = self.block(true)?;

        self.symtab.level_down();

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
                            self.source.id,
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
                            if let Type::Function { params, returns } = sym.ty.clone() {
                                sym.ty = Type::Function {
                                    params,
                                    returns: Box::new(TypeExpression {
                                        span: returns.span,
                                        ty: return_ty.ty,
                                    }),
                                };
                            } else {
                                panic!("ICE: function type in symtab is not Type::Function");
                            }
                        }
                    }
                }
            } else {
                // If the body has no return statement, make sure there is no return type
                // annotation on the function
                if !matches!(return_ty.ty, Type::Any | Type::Null) {
                    return Err(ParseError::FunctionHasIncompatibleReturnType(
                        self.source.id,
                        return_ty.span.into(),
                        return_ty.ty,
                        body.span.past().into(),
                        Type::Null,
                    ));
                }
            }
        }

        Ok(Statement {
            stmt: Stmt::Function(Function {
                ty: TypeExpression {
                    ty: func_type,
                    span: spanned_name.span,
                },
                name: spanned_name,
                params,
                body: Box::new(body),
            }),
            span: Span::combine(&[start_span, right_paren_span]),
        })
    }

    fn var_decl(&mut self) -> Result<Statement, ParseError> {
        trace!("parse_var_decl");
        let let_token_span = self.bump()?.span;

        let binding = self.binding()?;

        let mut ty = binding.ty.clone();
        let mut expr = None;
        if self.at(TokenKind::Equal) {
            self.bump()?;
            let val = self.expr()?;
            if !type_compatible(&ty, &val.ty) {
                return Err(ParseError::IncompatibleTypes(
                    self.source.id,
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
            name: binding.name.clone(),
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
        trace!("parse_stmt");
        if self.at(TokenKind::LeftBrace) {
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
        trace!("parse_ret_stmt");
        let ret_span = self.expect(TokenKind::Ret)?.span;

        if self.symtab.at_global_scope() {
            return Err(ParseError::CantReturnFromGlobalScope(
                self.source.id,
                ret_span.into(),
            ));
        }

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
        trace!("parse_until_stmt");
        let until_token_span = self.expect(TokenKind::Until)?.span;

        let cond = self.expr()?;

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
        trace!("parse_if_stmt");
        let if_token_span = self.expect(TokenKind::If)?.span;

        //self.expect(TokenKind::LeftParen)?;
        let cond = self.expr()?;
        //self.expect(TokenKind::RightParen)?;

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
        trace!("parse_block");
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

    fn expr_stmt(&mut self) -> Result<Statement, ParseError> {
        trace!("parse_expr_stmt");
        let expr = self.expr()?;
        let semi = self.expect(TokenKind::Semicolon)?;
        Ok(Statement {
            span: Span::combine(&[expr.span, semi.span]),
            stmt: Stmt::Expr(expr),
        })
    }

    fn expr(&mut self) -> Result<Expression, ParseError> {
        trace!("parse_expr");
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
        trace!("parse_assignment");
        let expr = self.or()?;

        if self.at(TokenKind::Equal) {
            let error_source = self.source.id;
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
                                self.source.id,
                                expr.span.into(),
                                entry.ty.clone(),
                                rhs.span.into(),
                                rhs.ty,
                            ))
                        }
                    } else {
                        Err(ParseError::UndeclaredVariable(
                            self.source.id,
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
                Expr::IndexGet { lhs, index } => Ok(Expression {
                    span: Span::combine(&[expr.span, rhs.span]),
                    expr: Expr::IndexSet {
                        lhs,
                        index,
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
        trace!("parse_or");
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
        trace!("parse_and");
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
        trace!("parse_equality");
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
        trace!("parse_comparison");
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
        trace!("parse_term");
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
        trace!("parse_factor");
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
        trace!("parse_unary");
        if self.at_set(&[TokenKind::Bang, TokenKind::Minus]) {
            let op: Operator = self.bump()?.into();
            let rhs = self.unary()?;

            if op.op == Op::Not && rhs.ty != Type::Bool {
                return Err(ParseError::UnsupportedUnaryOperation(
                    self.source.id,
                    op.span.into(),
                    rhs.span.into(),
                    rhs.ty,
                ));
            }

            if op.op == Op::Sub && (rhs.ty != Type::Float && rhs.ty != Type::Int) {
                return Err(ParseError::UnsupportedUnaryOperation(
                    self.source.id,
                    op.span.into(),
                    rhs.span.into(),
                    rhs.ty,
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
            self.index()
        }
    }

    fn index(&mut self) -> Result<Expression, ParseError> {
        trace!("parse_index");
        let mut expr = self.call()?;
        if self.at(TokenKind::LeftBracket) {
            self.bump()?;
            let index_expr = self.expr()?;
            let end_span = self.expect(TokenKind::RightBracket)?.span;

            // TODO type checking for indices

            expr = Expression {
                span: Span::combine(&[expr.span, end_span]),
                ty: Type::Any,
                expr: Expr::IndexGet {
                    lhs: Box::new(expr),
                    index: Box::new(index_expr),
                },
            }
        }

        Ok(expr)
    }

    fn call(&mut self) -> Result<Expression, ParseError> {
        trace!("parse_call");
        let mut expr = self.primary()?;

        loop {
            if self.at(TokenKind::LeftParen) {
                self.bump()?;
                expr = self.finish_call(expr)?;
            } else if self.at(TokenKind::Dot) {
                self.bump()?;
                let name = self.expect(TokenKind::Ident)?;

                // Can only use dot operator when left hand side is of type Instance or Class
                if !matches!(expr.ty, Type::Any | Type::Instance(..) | Type::Class(..)) {
                    return Err(ParseError::OnlyInstancesAndClassesHaveProperties(
                        self.source.id,
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
        trace!("parse_finish_call");
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

        trace!(format!("{:#?}", callee.expr));

        // TODO: type checking on get expr's for instances
        // Example: someInstance.foo()

        self.is_callable(&callee)?;

        let ty = match &callee.ty {
            Type::Class(class_name) => {
                // Type check class constructor
                let entry = self
                    .symtab
                    .lookup(class_name)
                    .expect("ICE: parser should have checked if class was declared by this point");
                if let Sym::Class { methods, .. } = &entry.sym {
                    if let Some(constructor) = methods.get("constructor") {
                        if let Type::Function { params, .. } = &constructor.ty {
                            for (param, arg) in params.iter().zip(args.iter()) {
                                if !type_compatible(&param.ty, &arg.ty) {
                                    return Err(ParseError::IncompatibleTypes(
                                        callee.span.source_id,
                                        param.span.into(),
                                        param.ty.clone(),
                                        arg.span.into(),
                                        arg.ty.clone(),
                                    ));
                                }
                            }
                        }
                    }

                    Type::Instance(class_name.to_string())
                } else {
                    panic!("ICE: callee symtab type and actual type are not the same");
                }
            }
            Type::Function { returns, .. } => returns.ty.clone(),
            _ => callee.ty.clone(),
        };

        Ok(Expression {
            span: Span::combine(&[callee.span, paren]),
            ty,
            expr: Expr::Call {
                callee: Box::new(callee),
                paren,
                args,
            },
        })
    }

    fn binding(&mut self) -> Result<Binding, ParseError> {
        trace!("parse_binding");
        let ident = self.expect(TokenKind::Ident)?;

        let name = ident.text.to_string();
        let span = ident.span;

        if let Some(sym) = self.symtab.lookup(&name) {
            return Err(ParseError::CannotRedeclareSymbol(
                self.source.id,
                sym.name.item.clone(),
                span.into(),
                sym.name.span.into(),
            ));
        }

        let mut ty = Type::Any;
        if self.at(TokenKind::Colon) {
            self.bump()?;

            ty = self.ty()?.ty;
        }

        Ok(Binding {
            name: Spanned { item: name, span },
            ty,
        })
    }

    fn ty(&mut self) -> Result<TypeExpression, ParseError> {
        trace!("parse_ty");
        let token = self.bump()?;
        let span = token.span;

        let ty = match token.kind {
            TokenKind::TypeInt => Type::Int,
            TokenKind::TypeFloat => Type::Float,
            TokenKind::TypeBool => Type::Bool,
            TokenKind::TypeString => Type::String,
            TokenKind::Null => Type::Null,
            TokenKind::TypeAny => Type::Any,
            TokenKind::Ident => Type::Instance(token.text.clone()),
            TokenKind::LeftBracket => {
                // array types. Ex: [int]
                let list_ty = self.ty()?;
                let end_span = self.expect(TokenKind::RightBracket)?.span;
                return Ok(TypeExpression {
                    ty: Type::List(Box::new(list_ty)),
                    span: Span::combine(&[span, end_span]),
                });
            }
            TokenKind::LeftParen => {
                // function type. Ex: (int, int) -> int
                let mut params = Vec::new();
                if !self.at(TokenKind::RightParen) {
                    loop {
                        let param = self.ty()?;
                        params.push(param);

                        if self.at(TokenKind::Comma) {
                            self.bump()?;
                        } else {
                            break;
                        }
                    }
                }

                self.expect(TokenKind::RightParen)?;

                // Function types must define return type
                self.expect(TokenKind::Arrow)?;
                let returns = self.ty()?;

                return Ok(TypeExpression {
                    span: Span::combine(&[span, returns.span]),
                    ty: Type::Function {
                        params,
                        returns: Box::new(returns),
                    },
                });
            }
            _ => return Err(ParseError::UnknownType(self.source.id, span.into())),
        };

        Ok(TypeExpression { ty, span })
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        trace!("parse_primary");
        trace!(format!("symtab before lookup: {:#?}", self.symtab.clone()));
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
            TokenKind::This => Ok(Expression {
                expr: Expr::This,
                span: token_span,
                ty: Type::Any,
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
                        self.source.id,
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
            TokenKind::LeftBracket => {
                let start_span = token.span;

                let mut exprs = vec![];
                if !self.at(TokenKind::RightBracket) {
                    loop {
                        let expr = self.expr()?;

                        // It's actually list shorthand expression
                        if self.at(TokenKind::Semicolon) {
                            self.bump()?;
                            let count = self.expr()?;

                            if !type_compatible(&count.ty, &Type::Int) {
                                return Err(ParseError::ListShorthandCountMustBeInt(
                                    count.span.source_id,
                                    count.span.into(),
                                ));
                            }

                            let end_span = self.expect(TokenKind::RightBracket)?.span;

                            return Ok(Expression {
                                ty: Type::List(Box::new(TypeExpression {
                                    ty: expr.ty.clone(),
                                    span: expr.span,
                                })),
                                expr: Expr::ListShorthand {
                                    value: Box::new(expr),
                                    count: Box::new(count),
                                },
                                span: Span::combine(&[start_span, end_span]),
                            });
                        } else {
                            exprs.push(expr);
                            if self.at(TokenKind::Comma) {
                                self.bump()?;
                            } else {
                                break;
                            }
                        }
                    }
                }

                let end_span = self.expect(TokenKind::RightBracket)?.span;

                // We do type checking on the array after creating it because we need the span of
                // the entire array for error messages
                let mut list_ty = TypeExpression {
                    ty: Type::Any,
                    span: Span::combine(&[start_span, end_span]),
                };
                for expr in &exprs {
                    if !type_compatible(&list_ty.ty, &expr.ty) {
                        return Err(ParseError::IncompatibleTypes(
                            expr.span.source_id,
                            list_ty.span.into(),
                            list_ty.ty,
                            expr.span.into(),
                            expr.ty.clone(),
                        ));
                    } else {
                        // coerce list type
                        list_ty.ty = expr.ty.clone();
                    }
                }

                Ok(Expression {
                    expr: Expr::List(exprs),
                    span: Span::combine(&[start_span, end_span]),
                    ty: Type::List(Box::new(list_ty)),
                })
            }
            _ => Err(ParseError::ExpectedExpression(
                self.source.id,
                token_span.into(),
            )),
        }
    }
}
