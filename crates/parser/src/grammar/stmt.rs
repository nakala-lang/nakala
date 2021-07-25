use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LetKw) {
        variable_def(p)
    } else if p.at(TokenKind::FnKw) {
        func::func(p)
    } else {
        expr::expr(p)
    }
}

fn variable_def(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::LetKw));
    let m = p.start();
    p.bump();

    p.expect(TokenKind::Ident);
    p.expect(TokenKind::Equals);

    expr::expr(p);

    Some(m.complete(p, SyntaxKind::VariableDef))
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_variable_definition() {
        check(
            "let foo = bar",
            expect![[r#"
Root@0..13
  VariableDef@0..13
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..7 "foo"
    Whitespace@7..8 " "
    Equals@8..9 "="
    Whitespace@9..10 " "
    VariableRef@10..13
      Ident@10..13 "bar""#]],
        );
    }

    #[test]
    fn recover_on_let_token() {
        check(
            "let a =\nlet b = a",
            expect![[r#"
                Root@0..17
                  VariableDef@0..8
                    LetKw@0..3 "let"
                    Whitespace@3..4 " "
                    Ident@4..5 "a"
                    Whitespace@5..6 " "
                    Equals@6..7 "="
                    Whitespace@7..8 "\n"
                  VariableDef@8..17
                    LetKw@8..11 "let"
                    Whitespace@11..12 " "
                    Ident@12..13 "b"
                    Whitespace@13..14 " "
                    Equals@14..15 "="
                    Whitespace@15..16 " "
                    VariableRef@16..17
                      Ident@16..17 "a"
                [31mParse Error[0m: at 8..11, expected [33mnumber[0m, [33mstring[0m, [33mboolean[0m, [33midentifier[0m, [33m-[0m, [33mnot[0m, [33m([0m, [33mcall[0m or [33m{[0m, but found [31mlet[0m"#]],
        );
    }
}
