use lexer::Token;
use syntax::SyntaxKind;
use text_size::TextRange;

pub(crate) struct Source<'t, 'input> {
    tokens: &'t [Token<'input>],
    cursor: usize,
}

impl<'t, 'input> Source<'t, 'input> {
    pub(crate) fn new(tokens: &'t [Token<'input>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub(crate) fn next_token(&mut self) -> Option<&'t Token<'input>> {
        self.eat_trivia();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub(crate) fn peek_kind(&mut self) -> Option<SyntaxKind> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    pub(crate) fn peek_token(&mut self) -> Option<&Token> {
        self.eat_trivia();
        self.peek_token_raw()
    }

    pub(crate) fn last_token_range(&self) -> Option<TextRange> {
        self.tokens.last().map(|Token { range, .. }| *range)
    }

    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, SyntaxKind::is_trivia)
    }

    fn peek_kind_raw(&self) -> Option<SyntaxKind> {
        self.tokens
            .get(self.cursor)
            .map(|Token { kind, .. }| (*kind).into())
    }

    fn peek_token_raw(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }
}
