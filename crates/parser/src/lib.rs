pub mod error;
mod parser;
pub mod source;

use crate::error::ParseError;
use crate::parser::Parser;
use crate::source::Source;
use ast::*;

#[derive(Debug, PartialEq)]
pub struct Parse {
    pub stmts: Vec<Stmt>,
}

pub fn parse(source: Source) -> Result<Parse, ParseError> {
    let mut parser = Parser::new(source);

    Ok(Parse {
        stmts: parser.program()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<'a> Into<Source<'a>> for &'a str {
        fn into(self) -> Source<'a> {
            Source::new(self.clone(), "".into())
        }
    }

    fn check(expected: Vec<Stmt>, actual: &str) {
        let result = parse(actual.into()).unwrap().stmts;
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_string() {
        check(
            vec![Stmt::Expr(Expr::Literal(Literal::String(
                "foo".to_string(),
            )))],
            r#""foo";"#,
        );
    }

    #[test]
    fn parse_true() {
        check(vec![Stmt::Expr(Expr::Literal(Literal::True))], "true;");
    }

    #[test]
    fn parse_false() {
        check(vec![Stmt::Expr(Expr::Literal(Literal::False))], "false;");
    }

    #[test]
    fn parse_null() {
        check(vec![Stmt::Expr(Expr::Literal(Literal::Null))], "null;");
    }

    #[test]
    fn parse_integer() {
        check(vec![Stmt::Expr(Expr::Literal(Literal::Number(5.0)))], "5;");
    }

    #[test]
    fn parse_float() {
        check(
            vec![Stmt::Expr(Expr::Literal(Literal::Number(1.23)))],
            "1.23;",
        );
    }

    #[test]
    fn parse_add() {
        check(
            vec![Stmt::Expr(Expr::Binary {
                lhs: Box::new(Expr::Literal(Literal::Number(1.0))),
                op: Op::Add,
                rhs: Box::new(Expr::Literal(Literal::Number(2.0))),
            })],
            "1 + 2;",
        );
    }

    #[test]
    fn parse_sub() {
        check(
            vec![Stmt::Expr(Expr::Binary {
                lhs: Box::new(Expr::Literal(Literal::Number(1.12))),
                op: Op::Sub,
                rhs: Box::new(Expr::Literal(Literal::Number(2.0))),
            })],
            "1.12 - 2;",
        )
    }

    #[test]
    fn parse_mul() {
        check(
            vec![Stmt::Expr(Expr::Binary {
                lhs: Box::new(Expr::Literal(Literal::Number(2.0))),
                op: Op::Mul,
                rhs: Box::new(Expr::Literal(Literal::Number(32.31))),
            })],
            "2 * 32.31;",
        );
    }

    #[test]
    fn parse_div() {
        check(
            vec![Stmt::Expr(Expr::Binary {
                lhs: Box::new(Expr::Literal(Literal::Number(100.5))),
                op: Op::Div,
                rhs: Box::new(Expr::Literal(Literal::Number(0.5))),
            })],
            "100.5 / 0.5;",
        );
    }

    #[test]
    fn parse_unary() {
        check(
            vec![Stmt::Expr(Expr::Unary {
                op: Op::Sub,
                rhs: Box::new(Expr::Literal(Literal::Number(123.4))),
            })],
            "-123.4;",
        );
    }

    #[test]
    fn parse_grouping() {
        check(
            vec![Stmt::Expr(Expr::Grouping(Box::new(Expr::Binary {
                lhs: Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(
                    1.0,
                ))))),
                op: Op::Add,
                rhs: Box::new(Expr::Unary {
                    op: Op::Sub,
                    rhs: Box::new(Expr::Literal(Literal::Number(42.3))),
                }),
            })))],
            "((1.0) + -42.3);",
        );
    }

    #[test]
    fn parse_not_equal() {
        check(
            vec![Stmt::Expr(Expr::Binary {
                lhs: Box::new(Expr::Literal(Literal::String("foo".to_string()))),
                op: Op::NotEquals,
                rhs: Box::new(Expr::Literal(Literal::String("bar".to_string()))),
            })],
            r#""foo" != "bar";"#,
        );
    }

    #[test]
    fn parse_equal() {
        check(
            vec![Stmt::Expr(Expr::Binary {
                lhs: Box::new(Expr::Literal(Literal::Number(42.0))),
                op: Op::Equals,
                rhs: Box::new(Expr::Literal(Literal::Null)),
            })],
            "42.0 == null;",
        );
    }

    #[test]
    fn parse_less_than() {
        check(
            vec![Stmt::Expr(Expr::Binary {
                lhs: Box::new(Expr::Literal(Literal::Number(10.0))),
                op: Op::LessThan,
                rhs: Box::new(Expr::Literal(Literal::Number(42.0))),
            })],
            "10.0 < 42.0;",
        );
    }

    #[test]
    fn parse_less_than_equals() {
        check(
            vec![Stmt::Expr(Expr::Binary {
                lhs: Box::new(Expr::Literal(Literal::Number(10.0))),
                op: Op::LessThanEquals,
                rhs: Box::new(Expr::Literal(Literal::Number(10.0))),
            })],
            "10.0 <= 10.0;",
        );
    }

    #[test]
    fn parse_variable_decl() {
        check(
            vec![Stmt::Variable {
                name: "something_Int3resting".to_string(),
                expr: None,
            }],
            "let something_Int3resting;",
        );
    }

    #[test]
    fn parse_variable_decl_with_init() {
        check(
            vec![Stmt::Variable {
                name: "x".to_string(),
                expr: Some(Expr::Variable("x".to_string())),
            }],
            "let x = x;",
        );
    }

    #[test]
    fn parse_variable_assignment() {
        check(
            vec![Stmt::Expr(Expr::Assign {
                name: "x".to_string(),
                rhs: Box::new(Expr::Literal(Literal::String("foobar".to_string()))),
            })],
            r#"x = "foobar";"#,
        );
    }

    #[test]
    fn parse_variable_expr() {
        check(
            vec![Stmt::Expr(Expr::Binary {
                lhs: Box::new(Expr::Literal(Literal::Number(100.0))),
                op: Op::Mul,
                rhs: Box::new(Expr::Variable("myVariable".to_string())),
            })],
            "100 * myVariable;",
        );
    }

    #[test]
    fn parse_empty_block() {
        check(vec![Stmt::Block(vec![])], "{}");
    }

    #[test]
    fn parse_simple_block() {
        check(
            vec![Stmt::Block(vec![
                Stmt::Variable {
                    name: "x".to_string(),
                    expr: Some(Expr::Literal(Literal::Number(1.0))),
                },
                Stmt::Expr(Expr::Binary {
                    lhs: Box::new(Expr::Literal(Literal::Number(1.0))),
                    op: Op::Add,
                    rhs: Box::new(Expr::Variable("x".to_string())),
                }),
            ])],
            "{let x = 1; 1 + x;}",
        )
    }

    #[test]
    fn parse_nested_block() {
        check(
            vec![Stmt::Block(vec![
                Stmt::Variable {
                    name: "x".to_string(),
                    expr: Some(Expr::Literal(Literal::Number(5.0))),
                },
                Stmt::Block(vec![Stmt::Expr(Expr::Binary {
                    lhs: Box::new(Expr::Variable("foo".to_string())),
                    op: Op::Sub,
                    rhs: Box::new(Expr::Variable("bar".to_string())),
                })]),
            ])],
            "{ let x = 5; { foo - bar; } }",
        )
    }
}
