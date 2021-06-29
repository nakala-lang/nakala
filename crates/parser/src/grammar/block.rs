use super::*;

pub(super) fn block(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::LBrace));

    let m = p.start();
    p.bump();

    loop {
        if p.at(TokenKind::RBrace) {
            p.bump();
            break;
        }

        if p.at_end() {
            // shouldn't have gotten here
            return None;
        }

        stmt::stmt(p);
    }

    Some(m.complete(p, SyntaxKind::CodeBlock))
}
