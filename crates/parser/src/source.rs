use lexer::{Lexer, Token, TokenKind};
use miette::{MietteSpanContents, NamedSource, SourceCode, SourceSpan};

#[derive(Debug, Clone)]
pub struct Source {
    id: usize,
    raw: String,
    name: String,
    tokens: Vec<Token>,
    cursor: usize,
}

impl Source {
    pub fn new(id: usize, raw: String, name: String) -> Self {
        let tokens: Vec<_> = Lexer::new(&raw).collect();
        Self {
            id,
            raw,
            name,
            tokens,
            cursor: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<&Token> {
        self.eat_trivia();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub fn peek_kind(&mut self) -> Option<TokenKind> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    pub fn eof(&self) -> SourceSpan {
        (self.raw.len() - 1, 0).into()
    }

    pub fn at_end(&self) -> bool {
        self.peek_token_raw().is_none()
    }

    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, TokenKind::is_trivia)
    }

    fn peek_kind_raw(&self) -> Option<TokenKind> {
        self.peek_token_raw().map(|Token { kind, .. }| *kind)
    }

    fn peek_token_raw(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }
}

impl SourceCode for Source {
    fn read_span<'a>(
        &'a self,
        span: &miette::SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        let contents = self
            .raw
            .read_span(span, context_lines_before, context_lines_after)?;
        Ok(Box::new(MietteSpanContents::new_named(
            self.name.clone(),
            contents.data(),
            *contents.span(),
            contents.line(),
            contents.column(),
            contents.line_count(),
        )))
    }
}

impl Into<NamedSource> for &Source {
    fn into(self) -> NamedSource {
        let name = self.name.clone();
        let input = self.raw.clone().to_string();

        NamedSource::new(name, input)
    }
}
