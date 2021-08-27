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
