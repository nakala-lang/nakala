use super::*;

pub(super) fn func(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::FnKw));
    let m = p.start();
    p.bump();

    p.expect(TokenKind::Ident);

    // parse param list
    param_ident_list(p);

    // function bodies are code blocks
    if expr::code_block(p).is_none() {
        p.error();
    }

    Some(m.complete(p, SyntaxKind::FunctionDef))
}

/// Parse the values in parameter list as identifiers
///
/// This is used when **declaring** functions
pub(crate) fn param_ident_list(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(TokenKind::LParen);

    while p.at(TokenKind::Ident) || p.at(TokenKind::Comma) {
        p.bump();
    }

    p.expect(TokenKind::RParen);

    Some(m.complete(p, SyntaxKind::ParamIdentList))
}

/// Parse the values in parameter list as expression
///
/// This is used when **calling** functions
pub(crate) fn param_value_list(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(TokenKind::LParen);

    let mut should_still_parse = true;
    while should_still_parse {
        if p.at(TokenKind::Comma) {
            p.bump();
            should_still_parse = true;
        } else if p.at(TokenKind::RParen) {
            p.bump();
            should_still_parse = false;
        } else {
            should_still_parse = expr::expr(p).is_some();
        }
    }

    Some(m.complete(p, SyntaxKind::ParamValueList))
}
