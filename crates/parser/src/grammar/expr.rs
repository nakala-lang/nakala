use crate::grammar::func::param_value_list;

use super::*;

pub(super) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    expr_binding_power(p, 0)
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) -> Option<CompletedMarker> {
    let mut lhs = lhs(p)?;

    loop {
        let op = if p.at(TokenKind::Plus) {
            BinaryOp::Add
        } else if p.at(TokenKind::Minus) {
            BinaryOp::Sub
        } else if p.at(TokenKind::Star) {
            BinaryOp::Mul
        } else if p.at(TokenKind::Slash) {
            BinaryOp::Div
        } else if p.at(TokenKind::GreaterThan) {
            BinaryOp::GreaterThan
        } else if p.at(TokenKind::GreaterThanOrEqual) {
            BinaryOp::GreaterThanOrEqual
        } else if p.at(TokenKind::LessThan) {
            BinaryOp::LessThan
        } else if p.at(TokenKind::LessThanOrEqual) {
            BinaryOp::LessThanOrEqual
        } else if p.at(TokenKind::OrKw) {
            BinaryOp::Or
        } else if p.at(TokenKind::AndKw) {
            BinaryOp::And
        } else if p.at(TokenKind::ComparisonEquals) {
            BinaryOp::ComparisonEquals
        } else {
            // We're not at an operator; we don't know what to do next, so we just return from
            // the function and let the caller decide
            break;
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }

        // Eat the operator token
        p.bump();

        let m = lhs.precede(p);
        let parsed_rhs = expr_binding_power(p, right_binding_power).is_some();
        lhs = m.complete(p, SyntaxKind::InfixExpr);

        if !parsed_rhs {
            break;
        }
    }

    Some(lhs)
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let cm = if p.at(TokenKind::Number) || p.at(TokenKind::String) || p.at(TokenKind::Boolean) {
        literal(p)
    } else if p.at(TokenKind::Ident) {
        variable_ref(p)
    } else if p.at(TokenKind::Minus) || p.at(TokenKind::NotKw) {
        prefix_expr(p)
    } else if p.at(TokenKind::LParen) {
        paren_expr(p)
    } else if p.at(TokenKind::CallKw) {
        function_call(p)
    } else if p.at(TokenKind::LBrace) {
        code_block(p)?
    } else {
        p.error();
        return None;
    };

    Some(cm)
}

fn function_call(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::CallKw));

    let m = p.start();
    p.bump();

    // get function ident
    p.expect(TokenKind::Ident);

    // parse param list
    param_value_list(p);

    m.complete(p, SyntaxKind::FunctionCall)
}

fn literal(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Number) || p.at(TokenKind::String) || p.at(TokenKind::Boolean));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::Literal)
}

fn variable_ref(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Ident));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::VariableRef)
}

fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Minus) || p.at(TokenKind::NotKw));

    let m = p.start();

    let op = if p.at(TokenKind::Minus) {
        UnaryOp::Neg
    } else {
        UnaryOp::Not
    };

    let ((), right_binding_power) = op.binding_power();

    p.bump();

    expr_binding_power(p, right_binding_power);

    m.complete(p, SyntaxKind::PrefixExpr)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::LParen));

    let m = p.start();
    p.bump();
    expr_binding_power(p, 0);
    p.expect(TokenKind::RParen);

    m.complete(p, SyntaxKind::ParenExpr)
}

pub(crate) fn code_block(p: &mut Parser) -> Option<CompletedMarker> {
    if !p.at(TokenKind::LBrace) {
        return None;
    }

    let m = p.start();
    p.bump();

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
            stmt::stmt(p);
        }
    }

    Some(m.complete(p, SyntaxKind::CodeBlock))
}

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    ComparisonEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
}

impl BinaryOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Or => (1, 2),
            Self::And => (3, 4),
            Self::ComparisonEquals
            | Self::GreaterThan
            | Self::GreaterThanOrEqual
            | Self::LessThan
            | Self::LessThanOrEqual => (5, 6),
            Self::Add | Self::Sub => (7, 8),
            Self::Mul | Self::Div => (9, 10),
        }
    }
}

enum UnaryOp {
    Neg,
    Not,
}

impl UnaryOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Not => ((), 4),
            Self::Neg => ((), 11),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
                Root@0..3
                  Literal@0..3
                    Number@0..3 "123""#]],
        )
    }

    #[test]
    fn parse_variable_ref() {
        check(
            "counter",
            expect![[r#"
                Root@0..7
                  VariableRef@0..7
                    Ident@0..7 "counter""#]],
        )
    }

    #[test]
    fn parse_simple_infix_expression() {
        check(
            "1+2",
            expect![[r#"
                Root@0..3
                  InfixExpr@0..3
                    Literal@0..1
                      Number@0..1 "1"
                    Plus@1..2 "+"
                    Literal@2..3
                      Number@2..3 "2""#]],
        )
    }

    #[test]
    fn parse_left_associative_infix_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
Root@0..7
  InfixExpr@0..7
    InfixExpr@0..5
      InfixExpr@0..3
        Literal@0..1
          Number@0..1 "1"
        Plus@1..2 "+"
        Literal@2..3
          Number@2..3 "2"
      Plus@3..4 "+"
      Literal@4..5
        Number@4..5 "3"
    Plus@5..6 "+"
    Literal@6..7
      Number@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
                Root@0..7
                  InfixExpr@0..7
                    InfixExpr@0..5
                      Literal@0..1
                        Number@0..1 "1"
                      Plus@1..2 "+"
                      InfixExpr@2..5
                        Literal@2..3
                          Number@2..3 "2"
                        Star@3..4 "*"
                        Literal@4..5
                          Number@4..5 "3"
                    Minus@5..6 "-"
                    Literal@6..7
                      Number@6..7 "4""#]],
        );
    }

    #[test]
    fn negation_has_higher_binding_power_than_binary_operators() {
        check(
            "-20+20",
            expect![[r#"
                Root@0..6
                  InfixExpr@0..6
                    PrefixExpr@0..3
                      Minus@0..1 "-"
                      Literal@1..3
                        Number@1..3 "20"
                    Plus@3..4 "+"
                    Literal@4..6
                      Number@4..6 "20""#]],
        );
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((((10))))))",
            expect![[r#"
                Root@0..14
                  ParenExpr@0..14
                    LParen@0..1 "("
                    ParenExpr@1..13
                      LParen@1..2 "("
                      ParenExpr@2..12
                        LParen@2..3 "("
                        ParenExpr@3..11
                          LParen@3..4 "("
                          ParenExpr@4..10
                            LParen@4..5 "("
                            ParenExpr@5..9
                              LParen@5..6 "("
                              Literal@6..8
                                Number@6..8 "10"
                              RParen@8..9 ")"
                            RParen@9..10 ")"
                          RParen@10..11 ")"
                        RParen@11..12 ")"
                      RParen@12..13 ")"
                    RParen@13..14 ")""#]],
        );
    }

    #[test]
    fn parentheses_affect_precedence() {
        check(
            "5*(2+1)",
            expect![[r#"
                Root@0..7
                  InfixExpr@0..7
                    Literal@0..1
                      Number@0..1 "5"
                    Star@1..2 "*"
                    ParenExpr@2..7
                      LParen@2..3 "("
                      InfixExpr@3..6
                        Literal@3..4
                          Number@3..4 "2"
                        Plus@4..5 "+"
                        Literal@5..6
                          Number@5..6 "1"
                      RParen@6..7 ")""#]],
        );
    }

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
                Root@0..7
                  Whitespace@0..3 "   "
                  Literal@3..7
                    Number@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
                Root@0..6
                  Literal@0..6
                    Number@0..3 "999"
                    Whitespace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
                Root@0..9
                  Whitespace@0..1 " "
                  Literal@1..9
                    Number@1..4 "123"
                    Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_infix_expression_interspersed_with_comments() {
        check(
            "
1
  + 1 # Add one
  + 10 # Add ten",
            expect![[r##"
                Root@0..35
                  Whitespace@0..1 "\n"
                  InfixExpr@1..35
                    InfixExpr@1..21
                      Literal@1..5
                        Number@1..2 "1"
                        Whitespace@2..5 "\n  "
                      Plus@5..6 "+"
                      Whitespace@6..7 " "
                      Literal@7..21
                        Number@7..8 "1"
                        Whitespace@8..9 " "
                        Comment@9..18 "# Add one"
                        Whitespace@18..21 "\n  "
                    Plus@21..22 "+"
                    Whitespace@22..23 " "
                    Literal@23..35
                      Number@23..25 "10"
                      Whitespace@25..26 " "
                      Comment@26..35 "# Add ten""##]],
        );
    }

    #[test]
    fn parse_infix_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
                Root@0..12
                  Whitespace@0..1 " "
                  InfixExpr@1..12
                    Literal@1..3
                      Number@1..2 "1"
                      Whitespace@2..3 " "
                    Plus@3..4 "+"
                    Whitespace@4..7 "   "
                    InfixExpr@7..12
                      Literal@7..8
                        Number@7..8 "2"
                      Star@8..9 "*"
                      Whitespace@9..10 " "
                      Literal@10..12
                        Number@10..11 "3"
                        Whitespace@11..12 " ""#]],
        );
    }

    #[test]
    fn parse_unclosed_parentheses() {
        check(
            "(foo",
            expect![[r#"
                Root@0..4
                  ParenExpr@0..4
                    LParen@0..1 "("
                    VariableRef@1..4
                      Ident@1..4 "foo"
                [31mParse Error[0m: at 1..4, expected [33m+[0m, [33m-[0m, [33m*[0m, [33m/[0m, [33m>[0m, [33m>=[0m, [33m<[0m, [33m<=[0m, [33mor[0m, [33mand[0m, [33m==[0m or [33m)[0m"#]],
        );
    }

    #[test]
    fn do_not_parse_operator_if_gettting_rhs_failed() {
        check(
            "(1+",
            expect![[r#"
                Root@0..3
                  ParenExpr@0..3
                    LParen@0..1 "("
                    InfixExpr@1..3
                      Literal@1..2
                        Number@1..2 "1"
                      Plus@2..3 "+"
                [31mParse Error[0m: at 2..3, expected [33mnumber[0m, [33mstring[0m, [33mboolean[0m, [33midentifier[0m, [33m-[0m, [33mnot[0m, [33m([0m, [33mcall[0m or [33m{[0m
                [31mParse Error[0m: at 2..3, expected [33m)[0m"#]],
        );
    }

    #[test]
    fn parse_code_block() {
        check(
            "{1+2}",
            expect![[r#"
Root@0..5
  CodeBlock@0..5
    LBrace@0..1 "{"
    InfixExpr@1..4
      Literal@1..2
        Number@1..2 "1"
      Plus@2..3 "+"
      Literal@3..4
        Number@3..4 "2"
    RBrace@4..5 "}""#]],
        );
    }

    #[test]
    fn parse_variable_definition_code_block() {
        check(
            "let x = {1+2}",
            expect![[r#"
Root@0..13
  VariableDef@0..13
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "x"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    CodeBlock@8..13
      LBrace@8..9 "{"
      InfixExpr@9..12
        Literal@9..10
          Number@9..10 "1"
        Plus@10..11 "+"
        Literal@11..12
          Number@11..12 "2"
      RBrace@12..13 "}""#]],
        );
    }

    #[test]
    fn parse_code_block_with_def_inside() {
        check(
            "let x = { let y = 10    y}",
            expect![[r#"
Root@0..26
  VariableDef@0..26
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "x"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    CodeBlock@8..26
      LBrace@8..9 "{"
      Whitespace@9..10 " "
      VariableDef@10..24
        LetKw@10..13 "let"
        Whitespace@13..14 " "
        Ident@14..15 "y"
        Whitespace@15..16 " "
        Equals@16..17 "="
        Whitespace@17..18 " "
        Literal@18..24
          Number@18..20 "10"
          Whitespace@20..24 "    "
      VariableRef@24..25
        Ident@24..25 "y"
      RBrace@25..26 "}""#]],
        );
    }

    #[test]
    fn parse_code_block_with_outside_reference() {
        check(
            "let z = 1  let x = { z + 5 }",
            expect![[r#"
Root@0..28
  VariableDef@0..11
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "z"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    Literal@8..11
      Number@8..9 "1"
      Whitespace@9..11 "  "
  VariableDef@11..28
    LetKw@11..14 "let"
    Whitespace@14..15 " "
    Ident@15..16 "x"
    Whitespace@16..17 " "
    Equals@17..18 "="
    Whitespace@18..19 " "
    CodeBlock@19..28
      LBrace@19..20 "{"
      Whitespace@20..21 " "
      InfixExpr@21..27
        VariableRef@21..23
          Ident@21..22 "z"
          Whitespace@22..23 " "
        Plus@23..24 "+"
        Whitespace@24..25 " "
        Literal@25..27
          Number@25..26 "5"
          Whitespace@26..27 " "
      RBrace@27..28 "}""#]],
        );
    }

    #[test]
    fn do_not_parse_block_if_missing_closing_brace() {
        check(
            "let z = { x + 1",
            expect![[r#"
                Root@0..15
                  VariableDef@0..15
                    LetKw@0..3 "let"
                    Whitespace@3..4 " "
                    Ident@4..5 "z"
                    Whitespace@5..6 " "
                    Equals@6..7 "="
                    Whitespace@7..8 " "
                    CodeBlock@8..15
                      LBrace@8..9 "{"
                      Whitespace@9..10 " "
                      InfixExpr@10..15
                        VariableRef@10..12
                          Ident@10..11 "x"
                          Whitespace@11..12 " "
                        Plus@12..13 "+"
                        Whitespace@13..14 " "
                        Literal@14..15
                          Number@14..15 "1"
                [31mParse Error[0m: at 14..15, expected [33m+[0m, [33m-[0m, [33m*[0m, [33m/[0m, [33m>[0m, [33m>=[0m, [33m<[0m, [33m<=[0m, [33mor[0m, [33mand[0m, [33m==[0m, [33m+[0m, [33m-[0m, [33m*[0m, [33m/[0m, [33m>[0m, [33m>=[0m, [33m<[0m, [33m<=[0m, [33mor[0m, [33mand[0m, [33m==[0m or [33m}[0m"#]],
        );
    }

    #[test]
    fn parse_string_literal() {
        check(
            "\"Hello, world!\"",
            expect![[r#"
Root@0..15
  Literal@0..15
    String@0..15 "\"Hello, world!\"""#]],
        );
    }

    #[test]
    fn parse_var_def_string_literal() {
        check(
            "let x = \"Hello, world!\"",
            expect![[r#"
Root@0..23
  VariableDef@0..23
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "x"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    Literal@8..23
      String@8..23 "\"Hello, world!\"""#]],
        );
    }

    #[test]
    fn parse_block_with_string_literal() {
        check(
            "let y = { let x = 10   \"100\" }",
            expect![[r#"
                Root@0..30
                  VariableDef@0..30
                    LetKw@0..3 "let"
                    Whitespace@3..4 " "
                    Ident@4..5 "y"
                    Whitespace@5..6 " "
                    Equals@6..7 "="
                    Whitespace@7..8 " "
                    CodeBlock@8..30
                      LBrace@8..9 "{"
                      Whitespace@9..10 " "
                      VariableDef@10..23
                        LetKw@10..13 "let"
                        Whitespace@13..14 " "
                        Ident@14..15 "x"
                        Whitespace@15..16 " "
                        Equals@16..17 "="
                        Whitespace@17..18 " "
                        Literal@18..23
                          Number@18..20 "10"
                          Whitespace@20..23 "   "
                      Literal@23..29
                        String@23..28 "\"100\""
                        Whitespace@28..29 " "
                      RBrace@29..30 "}""#]],
        );
    }

    #[test]
    fn parse_simple_function_def() {
        check(
            "fn test() {}",
            expect![[r#"
            Root@0..12
              FunctionDef@0..12
                FnKw@0..2 "fn"
                Whitespace@2..3 " "
                Ident@3..7 "test"
                ParamIdentList@7..10
                  LParen@7..8 "("
                  RParen@8..9 ")"
                  Whitespace@9..10 " "
                CodeBlock@10..12
                  LBrace@10..11 "{"
                  RBrace@11..12 "}""#]],
        )
    }

    #[test]
    fn parse_single_param_function_def() {
        check(
            "fn test(x,y) {}",
            expect![[r#"
            Root@0..15
              FunctionDef@0..15
                FnKw@0..2 "fn"
                Whitespace@2..3 " "
                Ident@3..7 "test"
                ParamIdentList@7..13
                  LParen@7..8 "("
                  Ident@8..9 "x"
                  Comma@9..10 ","
                  Ident@10..11 "y"
                  RParen@11..12 ")"
                  Whitespace@12..13 " "
                CodeBlock@13..15
                  LBrace@13..14 "{"
                  RBrace@14..15 "}""#]],
        )
    }

    #[test]
    fn parse_many_param_function_def() {
        check("fn superLongFunction(someVariable1,              some_otherVariable2,                somethingElse, x) { let z = 10    z }", expect![[r#"
            Root@0..122
              FunctionDef@0..122
                FnKw@0..2 "fn"
                Whitespace@2..3 " "
                Ident@3..20 "superLongFunction"
                ParamIdentList@20..103
                  LParen@20..21 "("
                  Ident@21..34 "someVariable1"
                  Comma@34..35 ","
                  Whitespace@35..49 "              "
                  Ident@49..68 "some_otherVariable2"
                  Comma@68..69 ","
                  Whitespace@69..85 "                "
                  Ident@85..98 "somethingElse"
                  Comma@98..99 ","
                  Whitespace@99..100 " "
                  Ident@100..101 "x"
                  RParen@101..102 ")"
                  Whitespace@102..103 " "
                CodeBlock@103..122
                  LBrace@103..104 "{"
                  Whitespace@104..105 " "
                  VariableDef@105..119
                    LetKw@105..108 "let"
                    Whitespace@108..109 " "
                    Ident@109..110 "z"
                    Whitespace@110..111 " "
                    Equals@111..112 "="
                    Whitespace@112..113 " "
                    Literal@113..119
                      Number@113..115 "10"
                      Whitespace@115..119 "    "
                  VariableRef@119..121
                    Ident@119..120 "z"
                    Whitespace@120..121 " "
                  RBrace@121..122 "}""#]])
    }

    #[test]
    fn parse_simple_function_call() {
        check(
            "call unknown()",
            expect![[r#"
            Root@0..14
              FunctionCall@0..14
                CallKw@0..4 "call"
                Whitespace@4..5 " "
                Ident@5..12 "unknown"
                ParamValueList@12..14
                  LParen@12..13 "("
                  RParen@13..14 ")""#]],
        )
    }

    #[test]
    fn do_no_parse_function_call_if_missing_parens() {
        check(
            "call someFunction(5, 10",
            expect![[r#"
            Root@0..23
              FunctionCall@0..23
                CallKw@0..4 "call"
                Whitespace@4..5 " "
                Ident@5..17 "someFunction"
                ParamValueList@17..23
                  LParen@17..18 "("
                  Literal@18..19
                    Number@18..19 "5"
                  Comma@19..20 ","
                  Whitespace@20..21 " "
                  Literal@21..23
                    Number@21..23 "10"
            [31mParse Error[0m: at 21..23, expected [33m+[0m, [33m-[0m, [33m*[0m, [33m/[0m, [33m>[0m, [33m>=[0m, [33m<[0m, [33m<=[0m, [33mor[0m, [33mand[0m, [33m==[0m, [33m,[0m, [33m)[0m, [33mnumber[0m, [33mstring[0m, [33mboolean[0m, [33midentifier[0m, [33m-[0m, [33mnot[0m, [33m([0m, [33mcall[0m or [33m{[0m"#]],
        );
    }

    #[test]
    fn parse_single_param_function_call() {
        check(
            r#"call someOtherFunction("hello world")"#,
            expect![[r#"
            Root@0..37
              FunctionCall@0..37
                CallKw@0..4 "call"
                Whitespace@4..5 " "
                Ident@5..22 "someOtherFunction"
                ParamValueList@22..37
                  LParen@22..23 "("
                  Literal@23..36
                    String@23..36 "\"hello world\""
                  RParen@36..37 ")""#]],
        )
    }

    #[test]
    fn parse_expr_function_call() {
        check(
            "call add({let x = 1   let y = 2    x + y }, 52)",
            expect![[r#"
                Root@0..47
                  FunctionCall@0..47
                    CallKw@0..4 "call"
                    Whitespace@4..5 " "
                    Ident@5..8 "add"
                    ParamValueList@8..47
                      LParen@8..9 "("
                      CodeBlock@9..42
                        LBrace@9..10 "{"
                        VariableDef@10..22
                          LetKw@10..13 "let"
                          Whitespace@13..14 " "
                          Ident@14..15 "x"
                          Whitespace@15..16 " "
                          Equals@16..17 "="
                          Whitespace@17..18 " "
                          Literal@18..22
                            Number@18..19 "1"
                            Whitespace@19..22 "   "
                        VariableDef@22..35
                          LetKw@22..25 "let"
                          Whitespace@25..26 " "
                          Ident@26..27 "y"
                          Whitespace@27..28 " "
                          Equals@28..29 "="
                          Whitespace@29..30 " "
                          Literal@30..35
                            Number@30..31 "2"
                            Whitespace@31..35 "    "
                        InfixExpr@35..41
                          VariableRef@35..37
                            Ident@35..36 "x"
                            Whitespace@36..37 " "
                          Plus@37..38 "+"
                          Whitespace@38..39 " "
                          VariableRef@39..41
                            Ident@39..40 "y"
                            Whitespace@40..41 " "
                        RBrace@41..42 "}"
                      Comma@42..43 ","
                      Whitespace@43..44 " "
                      Literal@44..46
                        Number@44..46 "52"
                      RParen@46..47 ")""#]],
        )
    }

    #[test]
    fn parse_variable_def_to_function_call() {
        check(
            "let x = call someFunction(5,5)",
            expect![[r#"
            Root@0..30
              VariableDef@0..30
                LetKw@0..3 "let"
                Whitespace@3..4 " "
                Ident@4..5 "x"
                Whitespace@5..6 " "
                Equals@6..7 "="
                Whitespace@7..8 " "
                FunctionCall@8..30
                  CallKw@8..12 "call"
                  Whitespace@12..13 " "
                  Ident@13..25 "someFunction"
                  ParamValueList@25..30
                    LParen@25..26 "("
                    Literal@26..27
                      Number@26..27 "5"
                    Comma@27..28 ","
                    Literal@28..29
                      Number@28..29 "5"
                    RParen@29..30 ")""#]],
        )
    }

    #[test]
    fn parse_function_calls_in_paren_expresion() {
        check(
            "let z = 1 + ( call function(1,1) + 15)",
            expect![[r#"
            Root@0..38
              VariableDef@0..38
                LetKw@0..3 "let"
                Whitespace@3..4 " "
                Ident@4..5 "z"
                Whitespace@5..6 " "
                Equals@6..7 "="
                Whitespace@7..8 " "
                InfixExpr@8..38
                  Literal@8..10
                    Number@8..9 "1"
                    Whitespace@9..10 " "
                  Plus@10..11 "+"
                  Whitespace@11..12 " "
                  ParenExpr@12..38
                    LParen@12..13 "("
                    Whitespace@13..14 " "
                    InfixExpr@14..37
                      FunctionCall@14..33
                        CallKw@14..18 "call"
                        Whitespace@18..19 " "
                        Ident@19..27 "function"
                        ParamValueList@27..33
                          LParen@27..28 "("
                          Literal@28..29
                            Number@28..29 "1"
                          Comma@29..30 ","
                          Literal@30..31
                            Number@30..31 "1"
                          RParen@31..32 ")"
                          Whitespace@32..33 " "
                      Plus@33..34 "+"
                      Whitespace@34..35 " "
                      Literal@35..37
                        Number@35..37 "15"
                    RParen@37..38 ")""#]],
        )
    }

    #[test]
    fn parse_simple_and_expression() {
        check(
            "false and true",
            expect![[r#"
            Root@0..14
              InfixExpr@0..14
                Literal@0..6
                  Boolean@0..5 "false"
                  Whitespace@5..6 " "
                AndKw@6..9 "and"
                Whitespace@9..10 " "
                Literal@10..14
                  Boolean@10..14 "true""#]],
        )
    }

    #[test]
    fn parse_simple_not_expression() {
        check(
            "not false",
            expect![[r#"
            Root@0..9
              PrefixExpr@0..9
                NotKw@0..3 "not"
                Whitespace@3..4 " "
                Literal@4..9
                  Boolean@4..9 "false""#]],
        )
    }

    #[test]
    fn parse_simple_or_expression() {
        check(
            "false or true",
            expect![[r#"
            Root@0..13
              InfixExpr@0..13
                Literal@0..6
                  Boolean@0..5 "false"
                  Whitespace@5..6 " "
                OrKw@6..8 "or"
                Whitespace@8..9 " "
                Literal@9..13
                  Boolean@9..13 "true""#]],
        )
    }

    #[test]
    fn parse_simple_greater_than_expression() {
        check(
            "1 > 1",
            expect![[r#"
            Root@0..5
              InfixExpr@0..5
                Literal@0..2
                  Number@0..1 "1"
                  Whitespace@1..2 " "
                GreaterThan@2..3 ">"
                Whitespace@3..4 " "
                Literal@4..5
                  Number@4..5 "1""#]],
        )
    }

    #[test]
    fn parse_simple_greater_than_or_equal_expression() {
        check(
            "1 >= 1",
            expect![[r#"
            Root@0..6
              InfixExpr@0..6
                Literal@0..2
                  Number@0..1 "1"
                  Whitespace@1..2 " "
                GreaterThanOrEqual@2..4 ">="
                Whitespace@4..5 " "
                Literal@5..6
                  Number@5..6 "1""#]],
        )
    }

    #[test]
    fn parse_simple_less_than_expression() {
        check(
            "1 < 1",
            expect![[r#"
            Root@0..5
              InfixExpr@0..5
                Literal@0..2
                  Number@0..1 "1"
                  Whitespace@1..2 " "
                LessThan@2..3 "<"
                Whitespace@3..4 " "
                Literal@4..5
                  Number@4..5 "1""#]],
        )
    }

    #[test]
    fn parse_simple_less_than_or_equal_expression() {
        check(
            "1 <= 1",
            expect![[r#"
            Root@0..6
              InfixExpr@0..6
                Literal@0..2
                  Number@0..1 "1"
                  Whitespace@1..2 " "
                LessThanOrEqual@2..4 "<="
                Whitespace@4..5 " "
                Literal@5..6
                  Number@5..6 "1""#]],
        )
    }
}
