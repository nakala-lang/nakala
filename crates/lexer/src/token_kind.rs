use logos::Logos;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub enum TokenKind {
    #[regex("[ \n]+")]
    Whitespace,

    #[token("fn")]
    FnKw,

    #[token("call")]
    CallKw,

    #[token("let")]
    LetKw,

    #[regex("[A-Za-z][A-Za-z0-9_]*")]
    Ident,

    #[regex("[0-9]+")]
    Number,

    #[regex(r#""[^"]*""#)]
    String,

    #[regex("true|false")]
    Boolean,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token(">")]
    GreaterThan,

    #[token(">=")]
    GreaterThanOrEqual,

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanOrEqual,

    #[token("and")]
    AndKw,

    #[token("or")]
    OrKw,

    #[token("not")]
    NotKw,

    #[token("==")]
    ComparisonEquals,

    #[token(",")]
    Comma,

    #[regex("#.*")]
    Comment,

    #[error]
    Error,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Whitespace => "whitespace",
            Self::FnKw => "‘fn’",
            Self::CallKw => "‘call’",
            Self::LetKw => "‘let’",
            Self::Ident => "identifier",
            Self::Number => "number",
            Self::String => "string",
            Self::Boolean => "boolean",
            Self::Plus => "‘+’",
            Self::Minus => "‘-’",
            Self::Star => "‘*’",
            Self::Slash => "‘/’",
            Self::Equals => "‘=’",
            Self::LParen => "‘(’",
            Self::RParen => "‘)’",
            Self::LBrace => "‘{’",
            Self::RBrace => "‘}’",
            Self::GreaterThan => "‘>’",
            Self::GreaterThanOrEqual => "‘>=’",
            Self::LessThan => "‘<’",
            Self::LessThanOrEqual => "‘<=’",
            Self::AndKw => "‘and’",
            Self::OrKw => "‘or’",
            Self::NotKw => "‘not’",
            Self::ComparisonEquals => "‘==’",
            Self::Comma => "‘,’",
            Self::Comment => "comment",
            Self::Error => "an unrecognized token",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);

        let token = lexer.next().unwrap();
        assert_eq!(token.kind, kind);
        assert_eq!(token.text, input);
    }

    #[test]
    fn lex_spaces_and_newlines() {
        check("  \n", TokenKind::Whitespace);
    }

    #[test]
    fn lex_spaces() {
        check("   ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_fn_keyword() {
        check("fn", TokenKind::FnKw);
    }

    #[test]
    fn lex_call_keyword() {
        check("call", TokenKind::CallKw);
    }

    #[test]
    fn lex_let_keyword() {
        check("let", TokenKind::LetKw);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", TokenKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", TokenKind::Ident);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", TokenKind::Ident);
    }

    #[test]
    fn lex_single_char_identifier() {
        check("x", TokenKind::Ident);
    }

    #[test]
    fn lex_number() {
        check("123456", TokenKind::Number);
    }

    #[test]
    fn lex_false_boolean() {
        check("false", TokenKind::Boolean);
    }

    #[test]
    fn lex_true_boolean() {
        check("true", TokenKind::Boolean);
    }

    #[test]
    fn lex_plus() {
        check("+", TokenKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", TokenKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", TokenKind::Star);
    }

    #[test]
    fn lex_slash() {
        check("/", TokenKind::Slash);
    }

    #[test]
    fn lex_equals() {
        check("=", TokenKind::Equals);
    }

    #[test]
    fn lex_left_brace() {
        check("{", TokenKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", TokenKind::RBrace);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", TokenKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", TokenKind::RParen);
    }

    #[test]
    fn lex_greater_than() {
        check(">", TokenKind::GreaterThan);
    }

    #[test]
    fn lex_greater_than_or_equals() {
        check(">=", TokenKind::GreaterThanOrEqual);
    }

    #[test]
    fn lex_less_than() {
        check("<", TokenKind::LessThan);
    }

    #[test]
    fn lex_less_than_or_equals() {
        check("<=", TokenKind::LessThanOrEqual);
    }

    #[test]
    fn lex_and_keyword() {
        check("and", TokenKind::AndKw);
    }

    #[test]
    fn lex_or_keyword() {
        check("or", TokenKind::OrKw);
    }

    #[test]
    fn lex_not_keyword() {
        check("not", TokenKind::NotKw);
    }

    #[test]
    fn lex_comparison_equals() {
        check("==", TokenKind::ComparisonEquals);
    }

    #[test]
    fn lex_comma() {
        check(",", TokenKind::Comma);
    }

    #[test]
    fn lex_comment() {
        check("# foo", TokenKind::Comment);
    }
}
