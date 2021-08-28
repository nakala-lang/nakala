use super::*;

// Classes follow the following syntax
//
//  class Apple {
//      fields {
//          x,y,z
//      }
//
//      fn foo(this, z) {
//
//      }
//
//      fn bar(this, aaa) {
//
//      }
//  }
pub(super) fn class_def(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::ClassKw));
    let m = p.start();

    p.bump();
    p.expect(TokenKind::Ident);
    p.expect(TokenKind::LBrace);

    let mut should_still_parse = true;
    while should_still_parse {
        if p.at(TokenKind::RBrace) {
            p.bump();
            should_still_parse = false;
        } else if p.at_end() {
            // shouldn't have gotten here
            p.error();
            should_still_parse = false;
        } else if p.at(TokenKind::FieldsKw) {
            class_fields(p);
        } else {
            class_member(p);
        }
    }

    Some(m.complete(p, SyntaxKind::ClassDef))
}

fn class_member(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::FnKw));
    let m = p.start();
    p.bump();

    p.expect(TokenKind::Ident);

    // parse param list
    class_member_ident_list(p);

    // function bodies are code blocks
    if expr::code_block(p).is_none() {
        p.error();
    }

    m.complete(p, SyntaxKind::ClassMethod)
}

fn class_member_ident_list(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(TokenKind::LParen);

    while p.at(TokenKind::Ident) || p.at(TokenKind::Comma) {
        p.bump();
    }

    p.expect(TokenKind::RParen);

    Some(m.complete(p, SyntaxKind::ParamIdentList))
}

fn class_fields(p: &mut Parser) {
    assert!(p.at(TokenKind::FieldsKw));
    p.bump();
    p.expect(TokenKind::LBrace);

    let mut should_still_parse = true;
    while should_still_parse {
        if p.at(TokenKind::Ident) {
            let m = p.start();
            p.bump();
            m.complete(p, SyntaxKind::ClassField);
        } else if p.at(TokenKind::Comma) {
            p.bump();
        } else if p.at(TokenKind::RBrace) {
            should_still_parse = false;
            p.bump();
        } else {
            p.error();
        }
    }
}

// Classes are created like
//
// let x = new Apple(field1, field2, ...)
pub(super) fn class_create(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::NewKw));
    let m = p.start();
    p.bump();

    p.expect(TokenKind::Ident);
    func::param_value_list(p);

    m.complete(p, SyntaxKind::ClassCreate)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_simple_class_def() {
        check(
            "class Apple {}",
            expect![[r#"
            Root@0..14
              ClassDef@0..14
                ClassKw@0..5 "class"
                Whitespace@5..6 " "
                Ident@6..11 "Apple"
                Whitespace@11..12 " "
                LBrace@12..13 "{"
                RBrace@13..14 "}""#]],
        );
    }

    #[test]
    fn parse_class_def_with_fields() {
        check(
            "class Foo { fields {x,y, somethingElseHere} }",
            expect![[r#"
                Root@0..45
                  ClassDef@0..45
                    ClassKw@0..5 "class"
                    Whitespace@5..6 " "
                    Ident@6..9 "Foo"
                    Whitespace@9..10 " "
                    LBrace@10..11 "{"
                    Whitespace@11..12 " "
                    FieldsKw@12..18 "fields"
                    Whitespace@18..19 " "
                    LBrace@19..20 "{"
                    ClassField@20..21
                      Ident@20..21 "x"
                    Comma@21..22 ","
                    ClassField@22..23
                      Ident@22..23 "y"
                    Comma@23..24 ","
                    Whitespace@24..25 " "
                    ClassField@25..42
                      Ident@25..42 "somethingElseHere"
                    RBrace@42..43 "}"
                    Whitespace@43..44 " "
                    RBrace@44..45 "}""#]],
        )
    }

    #[test]
    fn do_not_parse_method_without_this() {
        check("class Foo { fn testing() { 5.0 }  fn somethingElse() { }  fn somethingElseAlso(x) { } }",
        expect![[r#"
            Root@0..87
              ClassDef@0..87
                ClassKw@0..5 "class"
                Whitespace@5..6 " "
                Ident@6..9 "Foo"
                Whitespace@9..10 " "
                LBrace@10..11 "{"
                Whitespace@11..12 " "
                ClassMethod@12..34
                  FnKw@12..14 "fn"
                  Whitespace@14..15 " "
                  Ident@15..22 "testing"
                  ParamIdentList@22..25
                    LParen@22..23 "("
                    RParen@23..24 ")"
                    Whitespace@24..25 " "
                  CodeBlock@25..34
                    LBrace@25..26 "{"
                    Whitespace@26..27 " "
                    Literal@27..31
                      Number@27..28 "5"
                      Dot@28..29 "."
                      Number@29..30 "0"
                      Whitespace@30..31 " "
                    RBrace@31..32 "}"
                    Whitespace@32..34 "  "
                ClassMethod@34..58
                  FnKw@34..36 "fn"
                  Whitespace@36..37 " "
                  Ident@37..50 "somethingElse"
                  ParamIdentList@50..53
                    LParen@50..51 "("
                    RParen@51..52 ")"
                    Whitespace@52..53 " "
                  CodeBlock@53..58
                    LBrace@53..54 "{"
                    Whitespace@54..55 " "
                    RBrace@55..56 "}"
                    Whitespace@56..58 "  "
                ClassMethod@58..86
                  FnKw@58..60 "fn"
                  Whitespace@60..61 " "
                  Ident@61..78 "somethingElseAlso"
                  ParamIdentList@78..82
                    LParen@78..79 "("
                    Ident@79..80 "x"
                    RParen@80..81 ")"
                    Whitespace@81..82 " "
                  CodeBlock@82..86
                    LBrace@82..83 "{"
                    Whitespace@83..84 " "
                    RBrace@84..85 "}"
                    Whitespace@85..86 " "
                RBrace@86..87 "}""#]])
    }
}
