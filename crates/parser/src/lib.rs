pub mod error;
mod parser;
pub mod source;
mod symtab;

use crate::parser::Parser;
use crate::source::Source;
use ast::stmt::Statement;
pub use symtab::{Sym, Symbol, SymbolTable};

#[derive(Debug, PartialEq)]
pub struct Parse {
    pub stmts: Vec<Statement>,
    pub symtab: SymbolTable,
}

pub fn parse(source: Source, symtab: SymbolTable) -> miette::Result<Parse> {
    Parser::new(source, symtab).parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    impl<'a> Into<Source> for &'a str {
        fn into(self) -> Source {
            Source::new(0, String::from(self), "".into())
        }
    }

    fn check(actual: &str, expected: Expect) {
        let result = format!(
            "{:#?}",
            parse(actual.into(), SymbolTable::new(vec![])).unwrap()
        );
        expected.assert_eq(result.as_str())
    }

    #[test]
    fn parse_string() {
        check(
            r#""foo";"#,
            expect![[r#"
                Parse {
                    stmts: [
                        Statement {
                            stmt: Expr(
                                Expression {
                                    expr: String(
                                        "foo",
                                    ),
                                    span: Span {
                                        source_id: 0,
                                        start: 0,
                                        end: 5,
                                    },
                                    ty: String,
                                },
                            ),
                            span: Span {
                                source_id: 0,
                                start: 0,
                                end: 6,
                            },
                        },
                    ],
                    symtab: SymbolTable {
                        inner: [
                            {},
                        ],
                    },
                }"#]],
        );
    }

    #[test]
    fn parse_true() {
        check(
            "true;",
            expect![[r#"
                Parse {
                    stmts: [
                        Statement {
                            stmt: Expr(
                                Expression {
                                    expr: Bool(
                                        true,
                                    ),
                                    span: Span {
                                        source_id: 0,
                                        start: 0,
                                        end: 4,
                                    },
                                    ty: Bool,
                                },
                            ),
                            span: Span {
                                source_id: 0,
                                start: 0,
                                end: 5,
                            },
                        },
                    ],
                    symtab: SymbolTable {
                        inner: [
                            {},
                        ],
                    },
                }"#]],
        );
    }

    #[test]
    fn parse_false() {
        check(
            "false;",
            expect![[r#"
                Parse {
                    stmts: [
                        Statement {
                            stmt: Expr(
                                Expression {
                                    expr: Bool(
                                        false,
                                    ),
                                    span: Span {
                                        source_id: 0,
                                        start: 0,
                                        end: 5,
                                    },
                                    ty: Bool,
                                },
                            ),
                            span: Span {
                                source_id: 0,
                                start: 0,
                                end: 6,
                            },
                        },
                    ],
                    symtab: SymbolTable {
                        inner: [
                            {},
                        ],
                    },
                }"#]],
        );
    }

    #[test]
    fn parse_null() {
        check(
            "null;",
            expect![[r#"
                Parse {
                    stmts: [
                        Statement {
                            stmt: Expr(
                                Expression {
                                    expr: Null,
                                    span: Span {
                                        source_id: 0,
                                        start: 0,
                                        end: 4,
                                    },
                                    ty: Null,
                                },
                            ),
                            span: Span {
                                source_id: 0,
                                start: 0,
                                end: 5,
                            },
                        },
                    ],
                    symtab: SymbolTable {
                        inner: [
                            {},
                        ],
                    },
                }"#]],
        );
    }

    #[test]
    fn parse_integer() {
        check(
            "5;",
            expect![[r#"
                Parse {
                    stmts: [
                        Statement {
                            stmt: Expr(
                                Expression {
                                    expr: Int(
                                        5,
                                    ),
                                    span: Span {
                                        source_id: 0,
                                        start: 0,
                                        end: 1,
                                    },
                                    ty: Int,
                                },
                            ),
                            span: Span {
                                source_id: 0,
                                start: 0,
                                end: 2,
                            },
                        },
                    ],
                    symtab: SymbolTable {
                        inner: [
                            {},
                        ],
                    },
                }"#]],
        );
    }

    #[test]
    fn parse_float() {
        check(
            "1.23;",
            expect![[r#"
                Parse {
                    stmts: [
                        Statement {
                            stmt: Expr(
                                Expression {
                                    expr: Float(
                                        1.23,
                                    ),
                                    span: Span {
                                        source_id: 0,
                                        start: 0,
                                        end: 4,
                                    },
                                    ty: Float,
                                },
                            ),
                            span: Span {
                                source_id: 0,
                                start: 0,
                                end: 5,
                            },
                        },
                    ],
                    symtab: SymbolTable {
                        inner: [
                            {},
                        ],
                    },
                }"#]],
        );
    }

    //#[test]
    //fn parse_add() {
    //    check(
    //        vec![Stmt::Expr(Expr::Binary {
    //            lhs: Box::new(Expr::Literal(Literal::Number(1.0))),
    //            op: Op::Add,
    //            rhs: Box::new(Expr::Literal(Literal::Number(2.0))),
    //        })],
    //        "1 + 2;",
    //    );
    //}

    //#[test]
    //fn parse_sub() {
    //    check(
    //        vec![Stmt::Expr(Expr::Binary {
    //            lhs: Box::new(Expr::Literal(Literal::Number(1.12))),
    //            op: Op::Sub,
    //            rhs: Box::new(Expr::Literal(Literal::Number(2.0))),
    //        })],
    //        "1.12 - 2;",
    //    )
    //}

    //#[test]
    //fn parse_mul() {
    //    check(
    //        vec![Stmt::Expr(Expr::Binary {
    //            lhs: Box::new(Expr::Literal(Literal::Number(2.0))),
    //            op: Op::Mul,
    //            rhs: Box::new(Expr::Literal(Literal::Number(32.31))),
    //        })],
    //        "2 * 32.31;",
    //    );
    //}

    //#[test]
    //fn parse_div() {
    //    check(
    //        vec![Stmt::Expr(Expr::Binary {
    //            lhs: Box::new(Expr::Literal(Literal::Number(100.5))),
    //            op: Op::Div,
    //            rhs: Box::new(Expr::Literal(Literal::Number(0.5))),
    //        })],
    //        "100.5 / 0.5;",
    //    );
    //}

    //#[test]
    //fn parse_unary() {
    //    check(
    //        vec![Stmt::Expr(Expr::Unary {
    //            op: Op::Sub,
    //            rhs: Box::new(Expr::Literal(Literal::Number(123.4))),
    //        })],
    //        "-123.4;",
    //    );
    //}

    //#[test]
    //fn parse_grouping() {
    //    check(
    //        vec![Stmt::Expr(Expr::Grouping(Box::new(Expr::Binary {
    //            lhs: Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(
    //                1.0,
    //            ))))),
    //            op: Op::Add,
    //            rhs: Box::new(Expr::Unary {
    //                op: Op::Sub,
    //                rhs: Box::new(Expr::Literal(Literal::Number(42.3))),
    //            }),
    //        })))],
    //        "((1.0) + -42.3);",
    //    );
    //}

    //#[test]
    //fn parse_not_equal() {
    //    check(
    //        vec![Stmt::Expr(Expr::Binary {
    //            lhs: Box::new(Expr::Literal(Literal::String("foo".to_string()))),
    //            op: Op::NotEquals,
    //            rhs: Box::new(Expr::Literal(Literal::String("bar".to_string()))),
    //        })],
    //        r#""foo" != "bar";"#,
    //    );
    //}

    //#[test]
    //fn parse_equal() {
    //    check(
    //        vec![Stmt::Expr(Expr::Binary {
    //            lhs: Box::new(Expr::Literal(Literal::Number(42.0))),
    //            op: Op::Equals,
    //            rhs: Box::new(Expr::Literal(Literal::Null)),
    //        })],
    //        "42.0 == null;",
    //    );
    //}

    //#[test]
    //fn parse_less_than() {
    //    check(
    //        vec![Stmt::Expr(Expr::Binary {
    //            lhs: Box::new(Expr::Literal(Literal::Number(10.0))),
    //            op: Op::LessThan,
    //            rhs: Box::new(Expr::Literal(Literal::Number(42.0))),
    //        })],
    //        "10.0 < 42.0;",
    //    );
    //}

    //#[test]
    //fn parse_less_than_equals() {
    //    check(
    //        vec![Stmt::Expr(Expr::Binary {
    //            lhs: Box::new(Expr::Literal(Literal::Number(10.0))),
    //            op: Op::LessThanEquals,
    //            rhs: Box::new(Expr::Literal(Literal::Number(10.0))),
    //        })],
    //        "10.0 <= 10.0;",
    //    );
    //}

    //#[test]
    //fn parse_variable_decl() {
    //    check(
    //        vec![Stmt::Variable {
    //            name: "something_Int3resting".to_string(),
    //            expr: None,
    //        }],
    //        "let something_Int3resting;",
    //    );
    //}

    //#[test]
    //fn parse_variable_decl_with_init() {
    //    check(
    //        vec![Stmt::Variable {
    //            name: "x".to_string(),
    //            expr: Some(Expr::Variable("x".to_string())),
    //        }],
    //        "let x = x;",
    //    );
    //}

    //#[test]
    //fn parse_variable_assignment() {
    //    check(
    //        vec![Stmt::Expr(Expr::Assign {
    //            name: "x".to_string(),
    //            rhs: Box::new(Expr::Literal(Literal::String("foobar".to_string()))),
    //        })],
    //        r#"x = "foobar";"#,
    //    );
    //}

    //#[test]
    //fn parse_variable_expr() {
    //    check(
    //        vec![Stmt::Expr(Expr::Binary {
    //            lhs: Box::new(Expr::Literal(Literal::Number(100.0))),
    //            op: Op::Mul,
    //            rhs: Box::new(Expr::Variable("myVariable".to_string())),
    //        })],
    //        "100 * myVariable;",
    //    );
    //}

    //#[test]
    //fn parse_empty_block() {
    //    check(vec![Stmt::Block(vec![])], "{}");
    //}

    //#[test]
    //fn parse_simple_block() {
    //    check(
    //        vec![Stmt::Block(vec![
    //            Stmt::Variable {
    //                name: "x".to_string(),
    //                expr: Some(Expr::Literal(Literal::Number(1.0))),
    //            },
    //            Stmt::Expr(Expr::Binary {
    //                lhs: Box::new(Expr::Literal(Literal::Number(1.0))),
    //                op: Op::Add,
    //                rhs: Box::new(Expr::Variable("x".to_string())),
    //            }),
    //        ])],
    //        "{let x = 1; 1 + x;}",
    //    )
    //}

    //#[test]
    //fn parse_nested_block() {
    //    check(
    //        vec![Stmt::Block(vec![
    //            Stmt::Variable {
    //                name: "x".to_string(),
    //                expr: Some(Expr::Literal(Literal::Number(5.0))),
    //            },
    //            Stmt::Block(vec![Stmt::Expr(Expr::Binary {
    //                lhs: Box::new(Expr::Variable("foo".to_string())),
    //                op: Op::Sub,
    //                rhs: Box::new(Expr::Variable("bar".to_string())),
    //            })]),
    //        ])],
    //        "{ let x = 5; { foo - bar; } }",
    //    )
    //}

    //#[test]
    //fn parse_simple_if() {
    //    check(
    //        vec![Stmt::If {
    //            cond: Expr::Binary {
    //                lhs: Box::new(Expr::Variable("x".to_string())),
    //                op: Op::Equals,
    //                rhs: Box::new(Expr::Literal(Literal::Number(1.0))),
    //            },
    //            body: Box::new(Stmt::Block(vec![Stmt::Print(Expr::Literal(
    //                Literal::String("x is 1".to_string()),
    //            ))])),
    //            else_branch: None,
    //        }],
    //        r#"if (x == 1) { print "x is 1"; }"#,
    //    );
    //}

    //#[test]
    //fn parse_if_with_else() {
    //    check(
    //        vec![Stmt::If {
    //            cond: Expr::Literal(Literal::False),
    //            body: Box::new(Stmt::Block(vec![Stmt::Print(Expr::Literal(
    //                Literal::String("was false".to_string()),
    //            ))])),
    //            else_branch: Some(Box::new(Stmt::Block(vec![Stmt::Print(Expr::Literal(
    //                Literal::String("was true".to_string()),
    //            ))]))),
    //        }],
    //        r#"if (false) { print "was false"; } else { print "was true"; }"#,
    //    );
    //}

    //#[test]
    //fn parse_logical_and() {
    //    check(
    //        vec![Stmt::Expr(Expr::Logical {
    //            lhs: Box::new(Expr::Literal(Literal::False)),
    //            op: Op::And,
    //            rhs: Box::new(Expr::Literal(Literal::True)),
    //        })],
    //        "false and true;",
    //    );
    //}

    //#[test]
    //fn parse_logical_or() {
    //    check(
    //        vec![Stmt::Expr(Expr::Logical {
    //            lhs: Box::new(Expr::Literal(Literal::True)),
    //            op: Op::Or,
    //            rhs: Box::new(Expr::Variable("x".to_string())),
    //        })],
    //        "true or x;",
    //    );
    //}

    //#[test]
    //fn parse_until() {
    //    check(
    //        vec![Stmt::Until {
    //            cond: Expr::Literal(Literal::True),
    //            body: Box::new(Stmt::Block(vec![])),
    //        }],
    //        "until (true) { }",
    //    )
    //}

    //#[test]
    //fn parse_until_with_body() {
    //    check(
    //        vec![
    //            Stmt::Variable {
    //                name: "x".to_string(),
    //                expr: Some(Expr::Literal(Literal::Number(0.0))),
    //            },
    //            Stmt::Until {
    //                cond: Expr::Binary {
    //                    lhs: Box::new(Expr::Variable("x".to_string())),
    //                    op: Op::Equals,
    //                    rhs: Box::new(Expr::Literal(Literal::Number(10.0))),
    //                },
    //                body: Box::new(Stmt::Block(vec![
    //                    Stmt::Print(Expr::Literal(Literal::String("iter".to_string()))),
    //                    Stmt::Expr(Expr::Assign {
    //                        name: "x".to_string(),
    //                        rhs: Box::new(Expr::Binary {
    //                            lhs: Box::new(Expr::Variable("x".to_string())),
    //                            op: Op::Add,
    //                            rhs: Box::new(Expr::Literal(Literal::Number(1.0))),
    //                        }),
    //                    }),
    //                ])),
    //            },
    //        ],
    //        r#"let x = 0; until (x == 10) { print "iter"; x = x + 1; }"#,
    //    );
    //}
}
