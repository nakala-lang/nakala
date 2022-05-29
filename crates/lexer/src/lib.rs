use logos::Logos;
use meta::{SourceId, Span, Spanned};
use std::ops::Range as StdRange;

mod token_kind;
pub use token_kind::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: Span,
}

impl Into<Spanned<String>> for &Token {
    fn into(self) -> Spanned<String> {
        Spanned {
            item: self.text.clone(),
            span: self.span,
        }
    }
}

pub struct Lexer<'a> {
    source_id: SourceId,
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(source_id: SourceId, input: &'a str) -> Self {
        Self {
            source_id,
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        let span = {
            let StdRange { start, end } = self.inner.span();

            Span::new(self.source_id, start, end)
        };

        Some(Self::Item {
            kind,
            text: text.into(),
            span,
        })
    }
}
