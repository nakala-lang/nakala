use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    #[regex(r"[\s\t\n\f]+")] 
    Whitespace,

    // Single-character tokens
    #[token(")")]
    LeftParen,
    #[token("(")]
    RightParen,
    #[token("}")]
    LeftBrace,
    #[token("{")]
    RightBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token(";")]
    Semicolon,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,

    // One or more character tokens
    #[token("!")]
    Bang,
    #[token("!=")]
    BangEqual,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,

    // Literals
    #[regex("[A-Za-z_][A-Za-z0-9_]*")]
    Ident,
    #[regex(r#""[^"]*""#)]
    String,
    #[regex(r#"([0-9]+\.)?[0-9]+"#)]
    Number,

    // Keywords
    #[token("and")]
    And,
    #[token("class")]
    Class,
    #[token("else")]
    Else,
    #[token("false")]
    False,
    #[token("func")]
    Func,
    #[token("if")]
    If,
    #[token("null")]
    Null,
    #[token("or")]
    Or,
    #[token("print")]
    Print,
    #[token("ret")]
    Ret,
    #[token("super")]
    Super,
    #[token("this")]
    This,
    #[token("true")]
    True,
    #[token("let")]
    Let,
    #[token("until")]
    Until,

    #[error]
    Error,
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
        check("   \n", TokenKind::Whitespace);
    }

    #[test]
    fn lex_tabs_and_spaces() {
        check("\t ", TokenKind::Whitespace);
        check(" \t", TokenKind::Whitespace);
    }

    #[test]
    fn lex_left_paren() {
        check(")", TokenKind::LeftParen);
    }

    #[test]
    fn lex_right_paren() {
        check("(", TokenKind::RightParen);
    }

    #[test]
    fn lex_left_brace() {
        check("}", TokenKind::LeftBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("{", TokenKind::RightBrace);
    }

    #[test]
    fn lex_comma() {
        check(",", TokenKind::Comma);
    }

    #[test]
    fn lex_dot() {
        check(".", TokenKind::Dot);
    }

    #[test]
    fn lex_minus() {
        check("-", TokenKind::Minus);
    }

    #[test]
    fn lex_plus() {
        check("+", TokenKind::Plus);
    }

    #[test]
    fn lex_semicolon() {
        check(";", TokenKind::Semicolon);
    }

    #[test]
    fn lex_slash() {
        check("/", TokenKind::Slash);
    }

    #[test]
    fn lex_star() {
        check("*", TokenKind::Star);
    }
  
    #[test]
    fn lex_bang() {
        check("!", TokenKind::Bang);
    }

    #[test]
    fn lex_bang_equal() {
        check("!=", TokenKind::BangEqual);
    }

    #[test]
    fn lex_equal() {
        check("=", TokenKind::Equal);
    }
 
    #[test]
    fn lex_equal_equal() {
        check("==", TokenKind::EqualEqual);
    }
    
    #[test]
    fn lex_greater() {
        check(">", TokenKind::Greater);
    }

    #[test]
    fn lex_greater_equal() {
        check(">=", TokenKind::GreaterEqual);
    }

    #[test]
    fn lex_less() {
        check("<", TokenKind::Less);
    }
 
    #[test]
    fn lex_less_equal() {
        check("<=", TokenKind::LessEqual);
    }

    #[test]
    fn lex_simple_ident() {
        check("foo123", TokenKind::Ident);
    }

    #[test]
    fn lex_weird_ident() {
        check("A_91238i291_sdfa", TokenKind::Ident);
    }

    #[test]
    fn lex_leading_underscore_ident() {
        check("_foo", TokenKind::Ident);
    }

    #[test]
    fn lex_simple_string() {
        check(r#""foobar""#, TokenKind::String);
    }

    #[test]
    fn lex_weird_string() {
        check(r#""jfsdkaljf asdk kfjsd akfjsda asd fiasd""#, TokenKind::String);
    }

    #[test]
    fn lex_integer() {
        check("1", TokenKind::Number);
    }

    #[test]
    fn lex_float() {
        check("123.4", TokenKind::Number);
    }

    #[test]
    fn lex_zero_leading_float() {
        check("0.123", TokenKind::Number);
    }


    #[test]
    fn lex_and() {
        check("and", TokenKind::And);
    }

    #[test]
    fn lex_class() {
        check("class", TokenKind::Class);
    }

    #[test]
    fn lex_else() {
        check("else", TokenKind::Else);
    }

    #[test]
    fn lex_false() {
        check("false", TokenKind::False);
    }

    #[test]
    fn lex_func() {
        check("func", TokenKind::Func);
    }

    #[test]
    fn lex_if() {
        check("if", TokenKind::If);
    }

    #[test]
    fn lex_null() {
        check("null", TokenKind::Null);
    }

    #[test]
    fn lex_or() {
        check("or", TokenKind::Or);
    }

    #[test]
    fn lex_print() {
        check("print", TokenKind::Print);
    }

    #[test]
    fn lex_ret() {
        check("ret", TokenKind::Ret);
    }

    #[test]
    fn lex_super() {
        check("super", TokenKind::Super);
    }

    #[test]
    fn lex_this() {
        check("this", TokenKind::This);
    }

    #[test]
    fn lex_true() {
        check("true", TokenKind::True);
    }

    #[test]
    fn lex_let() {
        check("let", TokenKind::Let);
    }

    #[test]
    fn lex_until() {
        check("until", TokenKind::Until);
    }
}
