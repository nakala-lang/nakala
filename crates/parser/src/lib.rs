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

    #[test]
    fn parse_string() {
        let expected = vec![Stmt::Expr(Expr::Literal(Literal::String {
            val: "foo".to_string(),
        }))];
        let actual = parse(r#""foo";"#.into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_true() {
        let expected = vec![Stmt::Expr(Expr::Literal(Literal::True))];
        let actual = parse("true;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_false() {
        let expected = vec![Stmt::Expr(Expr::Literal(Literal::False))];
        let actual = parse("false;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_null() {
        let expected = vec![Stmt::Expr(Expr::Literal(Literal::Null))];
        let actual = parse("null;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_integer() {
        let expected = vec![Stmt::Expr(Expr::Literal(Literal::Number { val: 5.0 }))];
        let actual = parse("5;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_float() {
        let expected = vec![Stmt::Expr(Expr::Literal(Literal::Number { val: 1.23 }))];
        let actual = parse("1.23;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_add() {
        let expected = vec![Stmt::Expr(Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 1.0 })),
            op: Op::Add,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 2.0 })),
        })];
        let actual = parse("1 + 2;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_sub() {
        let expected = vec![Stmt::Expr(Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 1.12 })),
            op: Op::Sub,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 2.0 })),
        })];
        let actual = parse("1.12 - 2;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_mul() {
        let expected = vec![Stmt::Expr(Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 2.0 })),
            op: Op::Mul,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 32.31 })),
        })];
        let actual = parse("2 * 32.31;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_div() {
        let expected = vec![Stmt::Expr(Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 100.5 })),
            op: Op::Div,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 0.5 })),
        })];
        let actual = parse("100.5 / 0.5;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_unary() {
        let expected = vec![Stmt::Expr(Expr::Unary {
            op: Op::Sub,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 123.4 })),
        })];
        let actual = parse("-123.4;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_grouping() {
        let expected = vec![Stmt::Expr(Expr::Grouping(Box::new(Expr::Binary {
            lhs: Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number {
                val: 1.0,
            })))),
            op: Op::Add,
            rhs: Box::new(Expr::Unary {
                op: Op::Sub,
                rhs: Box::new(Expr::Literal(Literal::Number { val: 42.3 })),
            }),
        })))];
        let actual = parse("((1.0) + -42.3);".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_not_equal() {
        let expected = vec![Stmt::Expr(Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::String {
                val: "foo".to_string(),
            })),
            op: Op::NotEquals,
            rhs: Box::new(Expr::Literal(Literal::String {
                val: "bar".to_string(),
            })),
        })];
        let actual = parse(r#""foo" != "bar";"#.into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_equal() {
        let expected = vec![Stmt::Expr(Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 42.0 })),
            op: Op::Equals,
            rhs: Box::new(Expr::Literal(Literal::Null))
        })];
        let actual = parse("42.0 == null;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_less_than() {
        let expected = vec![Stmt::Expr(Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 10.0 })),
            op: Op::LessThan,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 42.0 })),
        })];

        let actual = parse("10.0 < 42.0;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_less_than_equals() {
        let expected = vec![Stmt::Expr(Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 10.0 })),
            op: Op::LessThanEquals,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 10.0 })),
        })];

        let actual = parse("10.0 <= 10.0;".into()).unwrap().stmts;
        assert_eq!(expected, actual);
    }

}
