use super::*;

pub(super) fn struct_def(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::StructKw));
    let m = p.start();
    p.bump();

    p.expect(TokenKind::Ident);

    // parse struct body
    struct_body(p);

    Some(m.complete(p, SyntaxKind::StructDef))
}

fn struct_body(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.expect(TokenKind::LBrace);

    let mut should_still_parse = true;
    while should_still_parse {
        if p.at(TokenKind::Comma) {
            p.bump();
        } else if p.at(TokenKind::RBrace) {
            p.bump();
            should_still_parse = false;
        } else if p.at_end() {
            // shouldn't have gotten here
            p.error();
            should_still_parse = false;
        } else {
            struct_member(p);
        }
    }

    m.complete(p, SyntaxKind::StructBody)
}

fn struct_member(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    p.expect(TokenKind::Ident);
    p.expect(TokenKind::Colon);

    // Until function defs can be expressed as expression values, we check both for struct members
    if p.at(TokenKind::FnKw) {
        func::func(p);
    } else {
        expr::expr(p);
    }

    m.complete(p, SyntaxKind::StructMemberDef)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_empty_body_def() {
        check(
            "struct t { }",
            expect![[r#"
            Root@0..12
              StructDef@0..12
                StructKw@0..6 "struct"
                Whitespace@6..7 " "
                Ident@7..8 "t"
                Whitespace@8..9 " "
                StructBody@9..12
                  LBrace@9..10 "{"
                  Whitespace@10..11 " "
                  RBrace@11..12 "}""#]],
        )
    }

    #[test]
    fn parse_simple_body_member() {
        check(
            "struct someStructDefinition {
                someStructMember     :       5
            }",
            expect![[r#"
                Root@0..90
                  StructDef@0..90
                    StructKw@0..6 "struct"
                    Whitespace@6..7 " "
                    Ident@7..27 "someStructDefinition"
                    Whitespace@27..28 " "
                    StructBody@28..90
                      LBrace@28..29 "{"
                      Whitespace@29..46 "\n                "
                      StructMemberDef@46..89
                        Ident@46..62 "someStructMember"
                        Whitespace@62..67 "     "
                        Colon@67..68 ":"
                        Whitespace@68..75 "       "
                        Literal@75..89
                          Number@75..76 "5"
                          Whitespace@76..89 "\n            "
                      RBrace@89..90 "}""#]],
        )
    }

    #[test]
    fn parse_multiple_members() {
        check(
            "struct testing {
                something: 1,
                somethingElse: { let x = 5   let z = 3.1    ret x + z },
                someOtherThing: fn test(x,y,z) {

                }
            }",
            expect![[r#"
                Root@0..201
                  StructDef@0..201
                    StructKw@0..6 "struct"
                    Whitespace@6..7 " "
                    Ident@7..14 "testing"
                    Whitespace@14..15 " "
                    StructBody@15..201
                      LBrace@15..16 "{"
                      Whitespace@16..33 "\n                "
                      StructMemberDef@33..45
                        Ident@33..42 "something"
                        Colon@42..43 ":"
                        Whitespace@43..44 " "
                        Literal@44..45
                          Number@44..45 "1"
                      Comma@45..46 ","
                      Whitespace@46..63 "\n                "
                      StructMemberDef@63..118
                        Ident@63..76 "somethingElse"
                        Colon@76..77 ":"
                        Whitespace@77..78 " "
                        CodeBlock@78..118
                          LBrace@78..79 "{"
                          Whitespace@79..80 " "
                          VariableDef@80..92
                            LetKw@80..83 "let"
                            Whitespace@83..84 " "
                            Ident@84..85 "x"
                            Whitespace@85..86 " "
                            Equals@86..87 "="
                            Whitespace@87..88 " "
                            Literal@88..92
                              Number@88..89 "5"
                              Whitespace@89..92 "   "
                          VariableDef@92..107
                            LetKw@92..95 "let"
                            Whitespace@95..96 " "
                            Ident@96..97 "z"
                            Whitespace@97..98 " "
                            Equals@98..99 "="
                            Whitespace@99..100 " "
                            Literal@100..107
                              Number@100..101 "3"
                              Dot@101..102 "."
                              Number@102..103 "1"
                              Whitespace@103..107 "    "
                          Return@107..117
                            RetKw@107..110 "ret"
                            Whitespace@110..111 " "
                            InfixExpr@111..117
                              VariableRef@111..113
                                Ident@111..112 "x"
                                Whitespace@112..113 " "
                              Plus@113..114 "+"
                              Whitespace@114..115 " "
                              VariableRef@115..117
                                Ident@115..116 "z"
                                Whitespace@116..117 " "
                          RBrace@117..118 "}"
                      Comma@118..119 ","
                      Whitespace@119..136 "\n                "
                      StructMemberDef@136..200
                        Ident@136..150 "someOtherThing"
                        Colon@150..151 ":"
                        Whitespace@151..152 " "
                        FunctionDef@152..200
                          FnKw@152..154 "fn"
                          Whitespace@154..155 " "
                          Ident@155..159 "test"
                          ParamIdentList@159..167
                            LParen@159..160 "("
                            Ident@160..161 "x"
                            Comma@161..162 ","
                            Ident@162..163 "y"
                            Comma@163..164 ","
                            Ident@164..165 "z"
                            RParen@165..166 ")"
                            Whitespace@166..167 " "
                          CodeBlock@167..200
                            LBrace@167..168 "{"
                            Whitespace@168..186 "\n\n                "
                            RBrace@186..187 "}"
                            Whitespace@187..200 "\n            "
                      RBrace@200..201 "}""#]],
        )
    }
}
