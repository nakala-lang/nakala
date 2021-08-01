use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LetKw) {
        variable_def(p)
    } else if p.at(TokenKind::FnKw) {
        func::func(p)
    } else if p.at(TokenKind::IfKw) {
        if_stmt(p)
    } else {
        // variable assignments can look like expressions,
        // since you could have x + 1 for example. Therefore,
        // we have to peek for the following signature to be sure
        //   <ident> <equals>
        if p.peek_multiple(vec![TokenKind::Ident, TokenKind::Equals]) {
            variable_assign(p)
        } else {
            expr::expr(p)
        }
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

fn if_stmt(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::IfKw));
    let m = p.start();
    p.bump();

    expr::expr(p);

    if expr::code_block(p).is_none() {
        p.error();
        Some(m.complete(p, SyntaxKind::Error))
    } else {
        Some(m.complete(p, SyntaxKind::If))
    }
}

fn variable_assign(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::Ident));
    let m = p.start();

    p.bump();
    p.expect(TokenKind::Equals);

    expr::expr(p);

    Some(m.complete(p, SyntaxKind::VariableAssign))
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
    fn parse_variable_assign() {
        check(
            "x = 5",
            expect![[r#"
            Root@0..5
              VariableAssign@0..5
                Ident@0..1 "x"
                Whitespace@1..2 " "
                Equals@2..3 "="
                Whitespace@3..4 " "
                Literal@4..5
                  Number@4..5 "5""#]],
        )
    }

    #[test]
    fn parse_if_statement() {
        check(
            "if true { 5 }",
            expect![[r#"
            Root@0..13
              If@0..13
                IfKw@0..2 "if"
                Whitespace@2..3 " "
                Literal@3..8
                  Boolean@3..7 "true"
                  Whitespace@7..8 " "
                CodeBlock@8..13
                  LBrace@8..9 "{"
                  Whitespace@9..10 " "
                  Literal@10..12
                    Number@10..11 "5"
                    Whitespace@11..12 " "
                  RBrace@12..13 "}""#]],
        )
    }

    #[test]
    fn if_stmt_recover_on_missing_block() {
        check(
            "if true or false",
            expect![[r#"
            Root@0..16
              Error@0..16
                IfKw@0..2 "if"
                Whitespace@2..3 " "
                InfixExpr@3..16
                  Literal@3..8
                    Boolean@3..7 "true"
                    Whitespace@7..8 " "
                  OrKw@8..10 "or"
                  Whitespace@10..11 " "
                  Literal@11..16
                    Boolean@11..16 "false"
            [31mParse Error[0m: at 11..16, expected [33m+[0m, [33m-[0m, [33m*[0m, [33m/[0m, [33m>[0m, [33m>=[0m, [33m<[0m, [33m<=[0m, [33mor[0m, [33mand[0m, [33m==[0m, [33m+[0m, [33m-[0m, [33m*[0m, [33m/[0m, [33m>[0m, [33m>=[0m, [33m<[0m, [33m<=[0m, [33mor[0m, [33mand[0m, [33m==[0m or [33m{[0m"#]],
        );
    }

    #[test]
    fn if_stmt_recover_on_missing_expr() {
        check(
            "if { let x = 5 }",
            expect![[r#"
            Root@0..16
              Error@0..16
                IfKw@0..2 "if"
                Whitespace@2..3 " "
                CodeBlock@3..16
                  LBrace@3..4 "{"
                  Whitespace@4..5 " "
                  VariableDef@5..15
                    LetKw@5..8 "let"
                    Whitespace@8..9 " "
                    Ident@9..10 "x"
                    Whitespace@10..11 " "
                    Equals@11..12 "="
                    Whitespace@12..13 " "
                    Literal@13..15
                      Number@13..14 "5"
                      Whitespace@14..15 " "
                  RBrace@15..16 "}"
            [31mParse Error[0m: at 15..16, expected [33m+[0m, [33m-[0m, [33m*[0m, [33m/[0m, [33m>[0m, [33m>=[0m, [33m<[0m, [33m<=[0m, [33mor[0m, [33mand[0m, [33m==[0m or [33m{[0m"#]],
        )
    }

    #[test]
    fn variable_assign_recover_on_missing_expr() {
        check(
            "let x = 5 x = let y = 100",
            expect![[r#"
            Root@0..25
              VariableDef@0..10
                LetKw@0..3 "let"
                Whitespace@3..4 " "
                Ident@4..5 "x"
                Whitespace@5..6 " "
                Equals@6..7 "="
                Whitespace@7..8 " "
                Literal@8..10
                  Number@8..9 "5"
                  Whitespace@9..10 " "
              VariableAssign@10..14
                Ident@10..11 "x"
                Whitespace@11..12 " "
                Equals@12..13 "="
                Whitespace@13..14 " "
              VariableDef@14..25
                LetKw@14..17 "let"
                Whitespace@17..18 " "
                Ident@18..19 "y"
                Whitespace@19..20 " "
                Equals@20..21 "="
                Whitespace@21..22 " "
                Literal@22..25
                  Number@22..25 "100"
            [31mParse Error[0m: at 14..17, expected [33mnumber[0m, [33mstring[0m, [33mboolean[0m, [33midentifier[0m, [33m-[0m, [33mnot[0m, [33m([0m, [33mcall[0m or [33m{[0m, but found [31mlet[0m"#]],
        )
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
