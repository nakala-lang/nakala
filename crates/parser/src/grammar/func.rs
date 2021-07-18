use super::*;

pub(super) fn func(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::FnKw));
    let m = p.start();
    p.bump();

    p.expect(TokenKind::Ident);

    // parse param list
    param_ident_list(p);

    // function bodies are code blocks
    if let None = expr::code_block(p) {
        dbg!("failed to parse code block!");
        p.error();
    }

    Some(m.complete(p, SyntaxKind::FunctionDef))
}

pub(crate) fn param_ident_list(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(TokenKind::LParen);

    while p.at(TokenKind::Ident) || p.at(TokenKind::Comma) {
        p.bump();
    }

    p.expect(TokenKind::RParen);

    Some(m.complete(p, SyntaxKind::ParamIdentList))
}
