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
        check("class Foo { fn testing(this) { 5.0 }  fn somethingElse() { }  fn somethingElseAlso(x) { } }",
        expect![[r#"
            Root@0..91
              ClassDef@0..91
                ClassKw@0..5 "class"
                Whitespace@5..6 " "
                Ident@6..9 "Foo"
                Whitespace@9..10 " "
                LBrace@10..11 "{"
                Whitespace@11..12 " "
                ClassMethod@12..38
                  FnKw@12..14 "fn"
                  Whitespace@14..15 " "
                  Ident@15..22 "testing"
                  ParamIdentList@22..29
                    LParen@22..23 "("
                    ThisKw@23..27 "this"
                    RParen@27..28 ")"
                    Whitespace@28..29 " "
                  CodeBlock@29..38
                    LBrace@29..30 "{"
                    Whitespace@30..31 " "
                    Literal@31..35
                      Number@31..32 "5"
                      Dot@32..33 "."
                      Number@33..34 "0"
                      Whitespace@34..35 " "
                    RBrace@35..36 "}"
                    Whitespace@36..38 "  "
                ClassMethod@38..62
                  FnKw@38..40 "fn"
                  Whitespace@40..41 " "
                  Ident@41..54 "somethingElse"
                  ParamIdentList@54..59
                    LParen@54..55 "("
                    Error@55..57
                      RParen@55..56 ")"
                      Whitespace@56..57 " "
                    Error@57..59
                      LBrace@57..58 "{"
                      Whitespace@58..59 " "
                  Error@59..62
                    RBrace@59..60 "}"
                    Whitespace@60..62 "  "
                ClassMethod@62..90
                  FnKw@62..64 "fn"
                  Whitespace@64..65 " "
                  Ident@65..82 "somethingElseAlso"
                  ParamIdentList@82..86
                    LParen@82..83 "("
                    Error@83..84
                      Ident@83..84 "x"
                    RParen@84..85 ")"
                    Whitespace@85..86 " "
                  CodeBlock@86..90
                    LBrace@86..87 "{"
                    Whitespace@87..88 " "
                    RBrace@88..89 "}"
                    Whitespace@89..90 " "
                RBrace@90..91 "}"
            [31mParse Error[0m: at 55..56, expected [33mthis[0m, but found [31m)[0m
            [31mParse Error[0m: at 57..58, expected [33midentifier[0m, [33m,[0m or [33m)[0m, but found [31m{[0m
            [31mParse Error[0m: at 59..60, expected [33m{[0m, but found [31m}[0m
            [31mParse Error[0m: at 83..84, expected [33mthis[0m, but found [31midentifier[0m"#]])
    }
}
