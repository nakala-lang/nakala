mod parser;
pub mod error;
pub mod source;

use crate::parser::Parser;
use crate::source::Source;
use crate::error::ParseError;
use ast::*;

#[derive(Debug, PartialEq)]
pub struct Parse {
    pub root: Expr
}

pub fn parse(source: Source) -> Result<Parse, ParseError> {
    let mut parser = Parser::new(source);
    
    Ok(Parse {
       root: parser.expr()?
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
    fn parse_integer() {
        let expected = Expr::Literal(Literal::Number { val: 5.0 });
        let actual = parse("5".into()).unwrap().root;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_float() {
        let expected = Expr::Literal(Literal::Number { val: 1.23 });
        let actual = parse("1.23".into()).unwrap().root;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_add() {
        let expected = Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 1.0 })),
            op: Op::Add,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 2.0 })),
        };
        let actual = parse("1 + 2".into()).unwrap().root;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_sub() {
        let expected = Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 1.12 })),
            op: Op::Sub,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 2.0 })),
        };
        let actual = parse("1.12 - 2".into()).unwrap().root;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_mul() {
        let expected = Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 2.0 })),
            op: Op::Mul,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 32.31 })),
        };
        let actual = parse("2 * 32.31".into()).unwrap().root;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_div() {
        let expected = Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 100.5 })),
            op: Op::Div,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 0.5 })),
        };
        let actual = parse("100.5 / 0.5".into()).unwrap().root;
        assert_eq!(expected, actual);
    }
}
