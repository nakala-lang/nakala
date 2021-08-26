use super::*;

// Classes follow the following syntax
//
//  class Apple {
//      fields {
//
//      }
//  }
//
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
        } else {
            class_member(p);
        }
    }

    Some(m.complete(p, SyntaxKind::ClassDef))
}

fn class_member(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    // Until function defs can be expressed as expresion
    m.complete(p, SyntaxKind::ClassMember)
}
