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

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_no_param_definition() {
        check(
            "fn test() { true }",
            expect![[r#"
            Root@0..18
              FunctionDef@0..18
                FnKw@0..2 "fn"
                Whitespace@2..3 " "
                Ident@3..7 "test"
                ParamIdentList@7..10
                  LParen@7..8 "("
                  RParen@8..9 ")"
                  Whitespace@9..10 " "
                CodeBlock@10..18
                  LBrace@10..11 "{"
                  Whitespace@11..12 " "
                  Literal@12..17
                    Boolean@12..16 "true"
                    Whitespace@16..17 " "
                  RBrace@17..18 "}""#]],
        );
    }

    #[test]
    fn parse_signle_param_definition() {
        check(
            "fn single_param(x) { x >= 5 }",
            expect![[r#"
            Root@0..29
              FunctionDef@0..29
                FnKw@0..2 "fn"
                Whitespace@2..3 " "
                Ident@3..15 "single_param"
                ParamIdentList@15..19
                  LParen@15..16 "("
                  Ident@16..17 "x"
                  RParen@17..18 ")"
                  Whitespace@18..19 " "
                CodeBlock@19..29
                  LBrace@19..20 "{"
                  Whitespace@20..21 " "
                  InfixExpr@21..28
                    VariableRef@21..23
                      Ident@21..22 "x"
                      Whitespace@22..23 " "
                    GreaterThanOrEqual@23..25 ">="
                    Whitespace@25..26 " "
                    Literal@26..28
                      Number@26..27 "5"
                      Whitespace@27..28 " "
                  RBrace@28..29 "}""#]],
        );
    }

    #[test]
    fn parse_multiple_param_definition() {
        check(
            "fn multiple_param(x,    y   , z122jkfjdsaf) { false }",
            expect![[r#"
                Root@0..53
                  FunctionDef@0..53
                    FnKw@0..2 "fn"
                    Whitespace@2..3 " "
                    Ident@3..17 "multiple_param"
                    ParamIdentList@17..44
                      LParen@17..18 "("
                      Ident@18..19 "x"
                      Comma@19..20 ","
                      Whitespace@20..24 "    "
                      Ident@24..25 "y"
                      Whitespace@25..28 "   "
                      Comma@28..29 ","
                      Whitespace@29..30 " "
                      Ident@30..42 "z122jkfjdsaf"
                      RParen@42..43 ")"
                      Whitespace@43..44 " "
                    CodeBlock@44..53
                      LBrace@44..45 "{"
                      Whitespace@45..46 " "
                      Literal@46..52
                        Boolean@46..51 "false"
                        Whitespace@51..52 " "
                      RBrace@52..53 "}""#]],
        )
    }
}
