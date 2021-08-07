pub(crate) mod marker;
pub mod parse_error;

use crate::event::Event;
use crate::grammar;
use crate::source::Source;
use lexer::{Token, TokenKind};
use marker::Marker;
use std::mem;
use syntax::SyntaxKind;

use parse_error::ParseError;

const RECOVERY_SET: [TokenKind; 1] = [TokenKind::LetKw];

#[derive(Clone)]
pub(crate) struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
    expected_kinds: Vec<TokenKind>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub(crate) fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
            expected_kinds: Vec::new(),
        }
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    pub(crate) fn parse(mut self) -> Vec<Event> {
        grammar::root(&mut self);
        self.events
    }

    pub(crate) fn expect(&mut self, kind: TokenKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    pub(crate) fn error(&mut self) {
        let current_token = self.source.peek_token();

        let (found, range) = if let Some(Token { kind, range, .. }) = current_token {
            (Some(*kind), *range)
        } else {
            // If we're at the end of hte input we use the range of the last input
            (None, self.source.last_token_range().unwrap())
        };

        self.events.push(Event::Error(ParseError {
            expected: mem::take(&mut self.expected_kinds),
            found,
            range,
        }));

        if !self.at_set(&RECOVERY_SET) && !self.at_end() {
            let m = self.start();
            self.bump();
            m.complete(self, SyntaxKind::Error);
        }
    }

    pub(crate) fn bump(&mut self) {
        self.expected_kinds.clear();
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    pub(crate) fn at(&mut self, kind: TokenKind) -> bool {
        self.expected_kinds.push(kind);
        self.peek() == Some(kind)
    }

    fn at_set(&mut self, set: &[TokenKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    pub(crate) fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    pub(crate) fn clear_expected(&mut self) {
        self.expected_kinds.clear();
    }

    // FIXME
    //
    // Sometimes (particularly with variable assignments),
    // we need to peek more than one token. This probably
    // should be replaced with better parse functionality but
    // for now, this should be good enough
    pub(crate) fn peek_multiple(&mut self, kinds: Vec<TokenKind>) -> bool {
        let mut cloned_parser = self.clone();

        for kind in kinds.into_iter() {
            if cloned_parser.at_end() || !cloned_parser.at(kind) {
                return false;
            }

            cloned_parser.bump();
        }

        true
    }

    fn peek(&mut self) -> Option<TokenKind> {
        self.source.peek_kind()
    }
}
