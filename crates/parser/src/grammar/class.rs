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
    let m = p.start();

    func::func(p);

    m.complete(p, SyntaxKind::ClassMethod)
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
}
