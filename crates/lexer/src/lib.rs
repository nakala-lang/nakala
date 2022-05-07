use logos::Logos;
use miette::SourceSpan;
use std::ops::Range as StdRange;

mod token_kind;
pub use token_kind::TokenKind;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub span: SourceSpan,
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        let span = {
            let StdRange { start, end } = self.inner.span();

            SourceSpan::new(start.into(), (end - start).into())
        };

        Some(Self::Item { kind, text, span })
    }
}
