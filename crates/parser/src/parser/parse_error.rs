use lexer::TokenKind;
use nu_ansi_term::Color::{Red, Yellow};
use std::fmt;
use text_size::TextRange;

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub expected: Vec<TokenKind>,
    pub found: Option<TokenKind>,
    pub range: TextRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: at {}..{}, expected ",
            Red.paint("Parse Error"),
            u32::from(self.range.start()),
            u32::from(self.range.end()),
        )?;

        let num_expected = self.expected.len();
        let is_first = |idx| idx == 0;
        let is_last = |idx| idx == num_expected - 1;

        for (idx, expected_kind) in self.expected.iter().enumerate() {
            if is_first(idx) {
                write!(f, "{}", Yellow.paint(expected_kind.to_string()))?;
            } else if is_last(idx) {
                write!(f, " or {}", Yellow.paint(expected_kind.to_string()))?;
            } else {
                write!(f, ", {}", Yellow.paint(expected_kind.to_string()))?;
            }
        }

        if let Some(found) = self.found {
            write!(f, ", but found {}", Red.paint(found.to_string()))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range as StdRange;

    fn check(
        expected: Vec<TokenKind>,
        found: Option<TokenKind>,
        range: StdRange<u32>,
        output: &str,
    ) {
        let error = ParseError {
            expected,
            found,
            range: {
                let start = range.start.into();
                let end = range.end.into();
                TextRange::new(start, end)
            },
        };

        assert_eq!(format!("{}", error), output);
    }

    #[test]
    fn one_expected_did_find() {
        check(
            vec![TokenKind::Equals],
            Some(TokenKind::Ident),
            10..20,
            "\u{1b}[31mParse Error\u{1b}[0m: at 10..20, expected \u{1b}[33m=\u{1b}[0m, but found \u{1b}[31midentifier\u{1b}[0m"
        )
    }

    #[test]
    fn one_expected_did_not_find() {
        check(
            vec![TokenKind::RParen],
            None,
            5..6,
            "\u{1b}[31mParse Error\u{1b}[0m: at 5..6, expected \u{1b}[33m)\u{1b}[0m",
        );
    }

    #[test]
    fn two_expected_did_find() {
        check(
            vec![TokenKind::Plus, TokenKind::Minus],
            Some(TokenKind::Equals),
            0..1,
            "\u{1b}[31mParse Error\u{1b}[0m: at 0..1, expected \u{1b}[33m+\u{1b}[0m or \u{1b}[33m-\u{1b}[0m, but found \u{1b}[31m=\u{1b}[0m"
        );
    }

    #[test]
    fn multiple_expected_did_find() {
        check(
            vec![
                TokenKind::Number,
                TokenKind::Ident,
                TokenKind::Minus,
                TokenKind::LParen,
            ],
            Some(TokenKind::LetKw),
            100..105,
            "\u{1b}[31mParse Error\u{1b}[0m: at 100..105, expected \u{1b}[33mnumber\u{1b}[0m, \u{1b}[33midentifier\u{1b}[0m, \u{1b}[33m-\u{1b}[0m or \u{1b}[33m(\u{1b}[0m, but found \u{1b}[31mlet\u{1b}[0m",
        );
    }
}
