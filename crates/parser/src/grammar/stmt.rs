use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LetKw) {
        variable_def(p)
    } else if p.at(TokenKind::FnKw) {
        func::func(p)
    } else if p.at(TokenKind::IfKw) {
        if_stmt(p)
    } else if p.at(TokenKind::RetKw) {
        return_stmt(p)
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
        // at the end of the if's code block, there be else branches
        if p.peek_multiple(vec![TokenKind::ElseKw, TokenKind::IfKw]) {
            else_if(p);
        } else if p.at(TokenKind::ElseKw) {
            else_stmt(p);
        }

        Some(m.complete(p, SyntaxKind::If))
    }
}

fn else_if(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    p.bump();

    if_stmt(p);

    Some(m.complete(p, SyntaxKind::ElseIf))
}

fn else_stmt(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::ElseKw));
    let m = p.start();
    p.bump();

    if expr::code_block(p).is_none() {
        p.error();
        return Some(m.complete(p, SyntaxKind::Error));
    }

    Some(m.complete(p, SyntaxKind::Else))
}

fn return_stmt(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::RetKw));
    let m = p.start();
    p.bump();

    if p.at(TokenKind::RBrace) {
        return Some(m.complete(p, SyntaxKind::Return));
    }

    expr::expr(p);

    Some(m.complete(p, SyntaxKind::Return))
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
                [31mParse Error[0m: at 11..16, expected [33m{[0m"#]],
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
                [31mParse Error[0m: at 15..16, expected [33m{[0m"#]],
        )
    }

    #[test]
    fn parse_else_if_statement() {
        check(
            "if true { } else if false {}",
            expect![[r#"
            Root@0..28
              If@0..28
                IfKw@0..2 "if"
                Whitespace@2..3 " "
                Literal@3..8
                  Boolean@3..7 "true"
                  Whitespace@7..8 " "
                CodeBlock@8..12
                  LBrace@8..9 "{"
                  Whitespace@9..10 " "
                  RBrace@10..11 "}"
                  Whitespace@11..12 " "
                ElseIf@12..28
                  ElseKw@12..16 "else"
                  Whitespace@16..17 " "
                  If@17..28
                    IfKw@17..19 "if"
                    Whitespace@19..20 " "
                    Literal@20..26
                      Boolean@20..25 "false"
                      Whitespace@25..26 " "
                    CodeBlock@26..28
                      LBrace@26..27 "{"
                      RBrace@27..28 "}""#]],
        )
    }

    #[test]
    fn parse_else_if_with_statements() {
        check(
            "if x >= 5 {} else if false { let x = 5 x = 10 x = x * 10 }",
            expect![[r#"
                Root@0..58
                  If@0..58
                    IfKw@0..2 "if"
                    Whitespace@2..3 " "
                    InfixExpr@3..10
                      VariableRef@3..5
                        Ident@3..4 "x"
                        Whitespace@4..5 " "
                      GreaterThanOrEqual@5..7 ">="
                      Whitespace@7..8 " "
                      Literal@8..10
                        Number@8..9 "5"
                        Whitespace@9..10 " "
                    CodeBlock@10..13
                      LBrace@10..11 "{"
                      RBrace@11..12 "}"
                      Whitespace@12..13 " "
                    ElseIf@13..58
                      ElseKw@13..17 "else"
                      Whitespace@17..18 " "
                      If@18..58
                        IfKw@18..20 "if"
                        Whitespace@20..21 " "
                        Literal@21..27
                          Boolean@21..26 "false"
                          Whitespace@26..27 " "
                        CodeBlock@27..58
                          LBrace@27..28 "{"
                          Whitespace@28..29 " "
                          VariableDef@29..39
                            LetKw@29..32 "let"
                            Whitespace@32..33 " "
                            Ident@33..34 "x"
                            Whitespace@34..35 " "
                            Equals@35..36 "="
                            Whitespace@36..37 " "
                            Literal@37..39
                              Number@37..38 "5"
                              Whitespace@38..39 " "
                          VariableAssign@39..46
                            Ident@39..40 "x"
                            Whitespace@40..41 " "
                            Equals@41..42 "="
                            Whitespace@42..43 " "
                            Literal@43..46
                              Number@43..45 "10"
                              Whitespace@45..46 " "
                          VariableAssign@46..57
                            Ident@46..47 "x"
                            Whitespace@47..48 " "
                            Equals@48..49 "="
                            Whitespace@49..50 " "
                            InfixExpr@50..57
                              VariableRef@50..52
                                Ident@50..51 "x"
                                Whitespace@51..52 " "
                              Star@52..53 "*"
                              Whitespace@53..54 " "
                              Literal@54..57
                                Number@54..56 "10"
                                Whitespace@56..57 " "
                          RBrace@57..58 "}""#]],
        )
    }

    #[test]
    fn else_if_recover_on_missing_expr() {
        check(
            "if true {} else if {}",
            expect![[r#"
                Root@0..21
                  If@0..21
                    IfKw@0..2 "if"
                    Whitespace@2..3 " "
                    Literal@3..8
                      Boolean@3..7 "true"
                      Whitespace@7..8 " "
                    CodeBlock@8..11
                      LBrace@8..9 "{"
                      RBrace@9..10 "}"
                      Whitespace@10..11 " "
                    ElseIf@11..21
                      ElseKw@11..15 "else"
                      Whitespace@15..16 " "
                      Error@16..21
                        IfKw@16..18 "if"
                        Whitespace@18..19 " "
                        CodeBlock@19..21
                          LBrace@19..20 "{"
                          RBrace@20..21 "}"
                [31mParse Error[0m: at 20..21, expected [33m{[0m"#]],
        );
    }

    #[test]
    fn else_if_recover_on_missing_block() {
        check(
            "if true {} else if false",
            expect![[r#"
                Root@0..24
                  If@0..24
                    IfKw@0..2 "if"
                    Whitespace@2..3 " "
                    Literal@3..8
                      Boolean@3..7 "true"
                      Whitespace@7..8 " "
                    CodeBlock@8..11
                      LBrace@8..9 "{"
                      RBrace@9..10 "}"
                      Whitespace@10..11 " "
                    ElseIf@11..24
                      ElseKw@11..15 "else"
                      Whitespace@15..16 " "
                      Error@16..24
                        IfKw@16..18 "if"
                        Whitespace@18..19 " "
                        Literal@19..24
                          Boolean@19..24 "false"
                [31mParse Error[0m: at 19..24, expected [33m{[0m"#]],
        )
    }

    #[test]
    fn else_if_recover_on_missing_block_closing_brace() {
        check(
            "if true {} else if false { let x = 5",
            expect![[r#"
                Root@0..36
                  If@0..36
                    IfKw@0..2 "if"
                    Whitespace@2..3 " "
                    Literal@3..8
                      Boolean@3..7 "true"
                      Whitespace@7..8 " "
                    CodeBlock@8..11
                      LBrace@8..9 "{"
                      RBrace@9..10 "}"
                      Whitespace@10..11 " "
                    ElseIf@11..36
                      ElseKw@11..15 "else"
                      Whitespace@15..16 " "
                      If@16..36
                        IfKw@16..18 "if"
                        Whitespace@18..19 " "
                        Literal@19..25
                          Boolean@19..24 "false"
                          Whitespace@24..25 " "
                        CodeBlock@25..36
                          LBrace@25..26 "{"
                          Whitespace@26..27 " "
                          VariableDef@27..36
                            LetKw@27..30 "let"
                            Whitespace@30..31 " "
                            Ident@31..32 "x"
                            Whitespace@32..33 " "
                            Equals@33..34 "="
                            Whitespace@34..35 " "
                            Literal@35..36
                              Number@35..36 "5"
                [31mParse Error[0m: at 35..36, expected [33m}[0m"#]],
        )
    }

    #[test]
    fn parse_else_statement() {
        check(
            "if true {} else { 5 }",
            expect![[r#"
            Root@0..21
              If@0..21
                IfKw@0..2 "if"
                Whitespace@2..3 " "
                Literal@3..8
                  Boolean@3..7 "true"
                  Whitespace@7..8 " "
                CodeBlock@8..11
                  LBrace@8..9 "{"
                  RBrace@9..10 "}"
                  Whitespace@10..11 " "
                Else@11..21
                  ElseKw@11..15 "else"
                  Whitespace@15..16 " "
                  CodeBlock@16..21
                    LBrace@16..17 "{"
                    Whitespace@17..18 " "
                    Literal@18..20
                      Number@18..19 "5"
                      Whitespace@19..20 " "
                    RBrace@20..21 "}""#]],
        )
    }

    #[test]
    fn parse_else_with_else_if() {
        check(
            "if true {} else if false {} else {}",
            expect![[r#"
            Root@0..35
              If@0..35
                IfKw@0..2 "if"
                Whitespace@2..3 " "
                Literal@3..8
                  Boolean@3..7 "true"
                  Whitespace@7..8 " "
                CodeBlock@8..11
                  LBrace@8..9 "{"
                  RBrace@9..10 "}"
                  Whitespace@10..11 " "
                ElseIf@11..35
                  ElseKw@11..15 "else"
                  Whitespace@15..16 " "
                  If@16..35
                    IfKw@16..18 "if"
                    Whitespace@18..19 " "
                    Literal@19..25
                      Boolean@19..24 "false"
                      Whitespace@24..25 " "
                    CodeBlock@25..28
                      LBrace@25..26 "{"
                      RBrace@26..27 "}"
                      Whitespace@27..28 " "
                    Else@28..35
                      ElseKw@28..32 "else"
                      Whitespace@32..33 " "
                      CodeBlock@33..35
                        LBrace@33..34 "{"
                        RBrace@34..35 "}""#]],
        );
    }

    #[test]
    fn else_recover_on_missing_block() {
        check(
            "if true {} else",
            expect![[r#"
            Root@0..15
              If@0..15
                IfKw@0..2 "if"
                Whitespace@2..3 " "
                Literal@3..8
                  Boolean@3..7 "true"
                  Whitespace@7..8 " "
                CodeBlock@8..11
                  LBrace@8..9 "{"
                  RBrace@9..10 "}"
                  Whitespace@10..11 " "
                Error@11..15
                  ElseKw@11..15 "else"
            [31mParse Error[0m: at 11..15, expected [33m{[0m"#]],
        )
    }

    #[test]
    fn else_recover_on_missing_block_closing_brace() {
        check(
            "if true {} else { let x = 5",
            expect![[r#"
                Root@0..27
                  If@0..27
                    IfKw@0..2 "if"
                    Whitespace@2..3 " "
                    Literal@3..8
                      Boolean@3..7 "true"
                      Whitespace@7..8 " "
                    CodeBlock@8..11
                      LBrace@8..9 "{"
                      RBrace@9..10 "}"
                      Whitespace@10..11 " "
                    Else@11..27
                      ElseKw@11..15 "else"
                      Whitespace@15..16 " "
                      CodeBlock@16..27
                        LBrace@16..17 "{"
                        Whitespace@17..18 " "
                        VariableDef@18..27
                          LetKw@18..21 "let"
                          Whitespace@21..22 " "
                          Ident@22..23 "x"
                          Whitespace@23..24 " "
                          Equals@24..25 "="
                          Whitespace@25..26 " "
                          Literal@26..27
                            Number@26..27 "5"
                [31mParse Error[0m: at 26..27, expected [33m}[0m"#]],
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
                [31mParse Error[0m: at 14..17, expected [33mnumber[0m, [33mstring[0m, [33mboolean[0m, [33midentifier[0m, [33m-[0m, [33mnot[0m, [33m([0m, [33mcall[0m, [33m[[0m or [33m{[0m, but found [31mlet[0m"#]],
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
                [31mParse Error[0m: at 8..11, expected [33mnumber[0m, [33mstring[0m, [33mboolean[0m, [33midentifier[0m, [33m-[0m, [33mnot[0m, [33m([0m, [33mcall[0m, [33m[[0m or [33m{[0m, but found [31mlet[0m"#]],
        );
    }
}
