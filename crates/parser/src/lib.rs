mod parser;
mod source;

use crate::parser::*;
use lexer::Lexer;
use source::Source;

#[derive(Debug, PartialEq)]
pub struct Parse {
    root: Expr
}

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let source = Source::new(&tokens);
    let mut parser = Parser::new(source);

    Parse {
        root: parser.expr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer() {
        let expected = Expr::Literal(Literal::Number { val: 5.0 });
        let actual = parse("5").root;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_float() {
        let expected = Expr::Literal(Literal::Number { val: 1.23 });
        let actual = parse("1.23").root;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_add() {
        let expected = Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 1.0 })),
            op: Op::Add,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 2.0 })),
        };
        let actual = parse("1 + 2").root;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_sub() {
        let expected = Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 1.12 })),
            op: Op::Sub,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 2.0 })),
        };
        let actual = parse("1.12 - 2").root;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_mul() {
        let expected = Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 2.0 })),
            op: Op::Mul,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 32.31 })),
        };
        let actual = parse("2 * 32.31").root;
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_div() {
        let expected = Expr::Binary {
            lhs: Box::new(Expr::Literal(Literal::Number { val: 100.5 })),
            op: Op::Div,
            rhs: Box::new(Expr::Literal(Literal::Number { val: 0.5 })),
        };
        let actual = parse("100.5 / 0.5").root;
        assert_eq!(expected, actual);
    }
}
