use crate::{
    BinaryOp, ClassDef, CodeBlock, Else, ElseBranch, ElseIf, Expr, FunctionDef, If, Return, Stmt,
    UnaryOp, VariableAssign, VariableDef,
};
use la_arena::Arena;
use syntax::SyntaxKind;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Database {
    pub exprs: Arena<Expr>,
}

impl Database {
    pub(crate) fn lower_stmt(&mut self, ast: ast::Stmt) -> Option<Stmt> {
        let result = match ast {
            ast::Stmt::VariableDef(ast) => Stmt::VariableDef(VariableDef {
                name: ast.name()?.text().into(),
                value: self.lower_expr(ast.value()),
            }),
            ast::Stmt::VariableAssign(ast) => Stmt::VariableAssign(VariableAssign {
                name: ast.name()?.text().into(),
                value: self.lower_expr(ast.value()),
            }),
            ast::Stmt::Expr(ast) => Stmt::Expr(self.lower_expr(Some(ast))),
            ast::Stmt::FunctionDef(ast) => Stmt::FunctionDef(self.lower_function_def(ast)?),
            ast::Stmt::If(ast) => Stmt::If(self.lower_if(ast)?),
            ast::Stmt::ElseIf(ast) => Stmt::ElseIf(ElseIf {
                if_stmt: self.lower_if(ast.if_stmt()?)?,
            }),
            ast::Stmt::Else(ast) => Stmt::Else(Else {
                body: self.lower_code_block(ast.body()?),
            }),
            ast::Stmt::Return(ast) => Stmt::Return(Return {
                value: ast.value().map(|_| self.lower_expr(ast.value())),
            }),
            ast::Stmt::ClassDef(ast) => Stmt::ClassDef(ClassDef {
                name: ast.name()?.text().into(),
                fields: ast.fields().into_iter().map(|i| i.text().into()).collect(),
                methods: ast
                    .methods()
                    .into_iter()
                    .filter_map(|func_def| self.lower_function_def(func_def))
                    .collect(),
            }),
        };

        Some(result)
    }

    pub(crate) fn lower_expr(&mut self, ast: Option<ast::Expr>) -> Expr {
        if let Some(ast) = ast {
            match ast {
                ast::Expr::BinaryExpr(ast) => self.lower_binary(ast),
                ast::Expr::Literal(ast) => self.lower_literal(ast),
                ast::Expr::ParenExpr(ast) => self.lower_expr(ast.expr()),
                ast::Expr::UnaryExpr(ast) => self.lower_unary(ast),
                ast::Expr::VariableRef(ast) => self.lower_variable_ref(ast),
                ast::Expr::CodeBlock(ast) => Expr::CodeBlock(self.lower_code_block(ast)),
                ast::Expr::FunctionCall(ast) => self.lower_function_call(ast),
                ast::Expr::List(ast) => self.lower_list(ast),
                ast::Expr::IndexOp(ast) => self.lower_index_op(ast),
                ast::Expr::ClassCreate(ast) => self.lower_class_create(ast),
            }
        } else {
            Expr::Missing
        }
    }

    fn lower_binary(&mut self, ast: ast::BinaryExpr) -> Expr {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Plus => BinaryOp::Add,
            SyntaxKind::Minus => BinaryOp::Sub,
            SyntaxKind::Star => BinaryOp::Mul,
            SyntaxKind::Slash => BinaryOp::Div,
            SyntaxKind::GreaterThan => BinaryOp::GreaterThan,
            SyntaxKind::GreaterThanOrEqual => BinaryOp::GreaterThanOrEqual,
            SyntaxKind::LessThan => BinaryOp::LessThan,
            SyntaxKind::LessThanOrEqual => BinaryOp::LessThanOrEqual,
            SyntaxKind::AndKw => BinaryOp::And,
            SyntaxKind::OrKw => BinaryOp::Or,
            SyntaxKind::ComparisonEquals => BinaryOp::ComparisonEquals,
            _ => unreachable!(),
        };

        let lhs = self.lower_expr(ast.lhs());
        let rhs = self.lower_expr(ast.rhs());

        Expr::Binary {
            op,
            lhs: self.exprs.alloc(lhs),
            rhs: self.exprs.alloc(rhs),
        }
    }

    fn lower_literal(&mut self, ast: ast::Literal) -> Expr {
        match ast {
            ast::Literal::Number(n) => Expr::Number { n: n.parse() },
            ast::Literal::String(s) => {
                // have to trim the leading and trailing " characters
                // they are on the node because the regex pattern captures them
                Expr::String {
                    s: s.parse()
                        .trim_start_matches('"')
                        .trim_end_matches('"')
                        .to_string(),
                }
            }
            ast::Literal::Boolean(b) => Expr::Boolean { b: b.parse() },
        }
    }

    fn lower_unary(&mut self, ast: ast::UnaryExpr) -> Expr {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Minus => UnaryOp::Neg,
            SyntaxKind::NotKw => UnaryOp::Not,
            _ => unreachable!(),
        };

        let expr = self.lower_expr(ast.expr());

        Expr::Unary {
            op,
            expr: self.exprs.alloc(expr),
        }
    }

    fn lower_variable_ref(&mut self, ast: ast::VariableRef) -> Expr {
        Expr::VariableRef {
            var: ast.name().unwrap().text().into(),
        }
    }

    fn lower_code_block(&mut self, ast: ast::CodeBlock) -> CodeBlock {
        let mut stmts = vec![];
        for stmt in ast.stmts() {
            if let Some(hir_stmt) = self.lower_stmt(stmt) {
                stmts.push(hir_stmt);
            }
        }
        CodeBlock { stmts }
    }

    fn lower_function_call(&mut self, ast: ast::FunctionCall) -> Expr {
        Expr::FunctionCall {
            name: ast
                .name()
                .expect("Failed to parse name of FunctionCall node")
                .text()
                .into(),
            param_value_list: ast
                .param_value_list()
                .into_iter()
                .map(|expr| self.lower_expr(Some(expr)))
                .collect(),
        }
    }

    fn lower_function_def(&mut self, ast: ast::FunctionDef) -> Option<FunctionDef> {
        Some(FunctionDef {
            name: ast.name()?.text().into(),
            param_ident_list: ast
                .param_ident_list()
                .into_iter()
                .map(|n| n.text().into())
                .collect(),
            body: self.lower_code_block(ast.body()?),
        })
    }

    fn lower_if(&mut self, if_stmt: ast::If) -> Option<If> {
        let expr = if_stmt.expr()?;
        let body = if_stmt.body()?;
        Some(If {
            expr: self.lower_expr(Some(expr)),
            body: self.lower_code_block(body),
            else_branch: match if_stmt.else_branch() {
                Some(branch) => self.lower_else_branch(branch).map(Box::new),
                None => None,
            },
        })
    }

    fn lower_else_branch(&mut self, else_branch: ast::ElseBranch) -> Option<ElseBranch> {
        Some(match else_branch {
            ast::ElseBranch::Else(else_ast) => ElseBranch::Else(Else {
                body: self.lower_code_block(else_ast.body()?),
            }),
            ast::ElseBranch::ElseIf(else_if_ast) => ElseBranch::ElseIf(ElseIf {
                if_stmt: self.lower_if(else_if_ast.if_stmt()?)?,
            }),
        })
    }

    fn lower_list(&mut self, list: ast::List) -> Expr {
        Expr::List {
            items: list
                .items()
                .into_iter()
                .map(|x| self.lower_expr(Some(x)))
                .collect(),
        }
    }

    fn lower_index_op(&mut self, index_op: ast::IndexOp) -> Expr {
        Expr::IndexOp {
            ident: index_op.ident().unwrap().text().into(),
            index: Box::new(self.lower_expr(index_op.index())),
        }
    }

    fn lower_class_create(&mut self, class_create: ast::ClassCreate) -> Expr {
        Expr::ClassCreate {
            name: class_create.name().unwrap().text().into(),
            param_value_list: class_create
                .param_value_list()
                .into_iter()
                .map(|expr| self.lower_expr(Some(expr)))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn check_stmt(input: &str, expected_hir: Stmt) {
        let root = parse(input);
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(hir, expected_hir);
    }

    fn check_expr(input: &str, expected_hir: Expr, expected_database: Database) {
        let root = parse(input);
        let first_stmt = root.stmts().next().unwrap();
        let ast = match first_stmt {
            ast::Stmt::Expr(ast) => ast,
            _ => unreachable!(),
        };
        let mut database = Database::default();
        let hir = database.lower_expr(Some(ast));

        assert_eq!(hir, expected_hir);
        assert_eq!(database, expected_database);
    }

    #[test]
    fn lower_variable_def() {
        let root = parse("let foo = bar");
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(
            hir,
            Stmt::VariableDef(VariableDef {
                name: "foo".into(),
                value: Expr::VariableRef { var: "bar".into() }
            })
        )
    }

    #[test]
    fn lower_variable_def_without_name() {
        let root = parse("let = 10");
        let ast = root.stmts().next().unwrap();
        assert!(Database::default().lower_stmt(ast).is_none());
    }

    #[test]
    fn lower_variable_def_without_value() {
        check_stmt(
            "let a =",
            Stmt::VariableDef(VariableDef {
                name: "a".into(),
                value: Expr::Missing,
            }),
        );
    }

    #[test]
    fn lower_expr_stmt() {
        check_stmt("123", Stmt::Expr(Expr::Number { n: 123.0 }));
    }

    #[test]
    fn lower_binary_expr() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Number { n: 1.2 });
        let rhs = exprs.alloc(Expr::Number { n: 2.0 });

        check_expr(
            "1.2+2",
            Expr::Binary {
                lhs,
                rhs,
                op: BinaryOp::Add,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_binary_expr_without_rhs() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Number { n: 10.0 });
        let rhs = exprs.alloc(Expr::Missing);

        check_expr(
            "10 -",
            Expr::Binary {
                lhs,
                rhs,
                op: BinaryOp::Sub,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_literal() {
        check_expr("999", Expr::Number { n: 999.0 }, Database::default());
    }

    #[test]
    fn lower_paren_expr() {
        check_expr(
            "((((((abc))))))",
            Expr::VariableRef { var: "abc".into() },
            Database::default(),
        )
    }

    #[test]
    fn lower_unary_expr() {
        let mut exprs = Arena::new();
        let ten = exprs.alloc(Expr::Number { n: 10.1 });

        check_expr(
            "-10.1",
            Expr::Unary {
                expr: ten,
                op: UnaryOp::Neg,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_unary_expr_without_expr() {
        let mut exprs = Arena::new();
        let expr = exprs.alloc(Expr::Missing);

        check_expr(
            "-",
            Expr::Unary {
                expr,
                op: UnaryOp::Neg,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_variable_ref() {
        check_expr(
            "foo",
            Expr::VariableRef { var: "foo".into() },
            Database::default(),
        )
    }

    #[test]
    fn lower_code_block() {
        check_expr(
            "{ 2 }",
            Expr::CodeBlock(CodeBlock {
                stmts: vec![Stmt::Expr(Expr::Number { n: 2.0 })],
            }),
            Database::default(),
        )
    }

    #[test]
    fn lower_function_def() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::VariableRef { var: "x".into() });
        let rhs = exprs.alloc(Expr::VariableRef { var: "y".into() });
        check_stmt(
            "fn add(x,y) { x + y }",
            Stmt::FunctionDef(FunctionDef {
                name: "add".into(),
                param_ident_list: vec!["x".into(), "y".into()],
                body: CodeBlock {
                    stmts: vec![Stmt::Expr(Expr::Binary {
                        lhs,
                        rhs,
                        op: BinaryOp::Add,
                    })],
                },
            }),
        )
    }

    #[test]
    fn lower_function_call() {
        check_expr(
            "call add(10, 5)",
            Expr::FunctionCall {
                name: "add".into(),
                param_value_list: vec![Expr::Number { n: 10.0 }, Expr::Number { n: 5.0 }],
            },
            Database::default(),
        );
    }

    #[test]
    fn lower_less_than() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Number { n: 5.0 });
        let rhs = exprs.alloc(Expr::Number { n: 2.0 });

        check_expr(
            "5 < 2",
            Expr::Binary {
                op: BinaryOp::LessThan,
                lhs,
                rhs,
            },
            Database { exprs },
        )
    }

    #[test]
    fn lower_less_than_or_equal() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Number { n: 5.0 });
        let rhs = exprs.alloc(Expr::Number { n: 2.0 });

        check_expr(
            "5 <= 2",
            Expr::Binary {
                op: BinaryOp::LessThanOrEqual,
                lhs,
                rhs,
            },
            Database { exprs },
        )
    }

    #[test]
    fn lower_greater_than() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Number { n: 5.0 });
        let rhs = exprs.alloc(Expr::Number { n: 2.0 });

        check_expr(
            "5 > 2",
            Expr::Binary {
                op: BinaryOp::GreaterThan,
                lhs,
                rhs,
            },
            Database { exprs },
        )
    }

    #[test]
    fn lower_greater_than_or_equal() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Number { n: 5.0 });
        let rhs = exprs.alloc(Expr::Number { n: 2.0 });

        check_expr(
            "5 >= 2",
            Expr::Binary {
                op: BinaryOp::GreaterThanOrEqual,
                lhs,
                rhs,
            },
            Database { exprs },
        )
    }

    #[test]
    fn lower_and() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Boolean { b: true });
        let rhs = exprs.alloc(Expr::Boolean { b: false });

        check_expr(
            "true and false",
            Expr::Binary {
                op: BinaryOp::And,
                lhs,
                rhs,
            },
            Database { exprs },
        )
    }

    #[test]
    fn lower_or() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Boolean { b: true });
        let rhs = exprs.alloc(Expr::Boolean { b: false });

        check_expr(
            "true or false",
            Expr::Binary {
                op: BinaryOp::Or,
                lhs,
                rhs,
            },
            Database { exprs },
        )
    }

    #[test]
    fn lower_not() {
        let mut exprs = Arena::new();
        let rhs = exprs.alloc(Expr::Boolean { b: true });

        check_expr(
            "not true",
            Expr::Unary {
                expr: rhs,
                op: UnaryOp::Not,
            },
            Database { exprs },
        )
    }

    #[test]
    fn lower_if() {
        let root = parse("if true { x = 5 }");
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(
            hir,
            Stmt::If(If {
                expr: Expr::Boolean { b: true },
                body: CodeBlock {
                    stmts: vec![Stmt::VariableAssign(VariableAssign {
                        name: "x".into(),
                        value: Expr::Number { n: 5.0 }
                    })]
                },
                else_branch: None
            })
        )
    }

    #[test]
    fn lower_else_if() {
        let root = parse("if true {1} else if false {2}");
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(
            hir,
            Stmt::If(If {
                expr: Expr::Boolean { b: true },
                body: CodeBlock {
                    stmts: vec![Stmt::Expr(Expr::Number { n: 1.0 })]
                },
                else_branch: Some(Box::new(ElseBranch::ElseIf(ElseIf {
                    if_stmt: If {
                        expr: Expr::Boolean { b: false },
                        body: CodeBlock {
                            stmts: vec![Stmt::Expr(Expr::Number { n: 2.0 })]
                        },
                        else_branch: None
                    }
                })))
            })
        )
    }

    #[test]
    fn lower_else_with_else_if() {
        let root = parse("if true {1} else if false {2} else {3}");
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(
            hir,
            Stmt::If(If {
                expr: Expr::Boolean { b: true },
                body: CodeBlock {
                    stmts: vec![Stmt::Expr(Expr::Number { n: 1.0 })]
                },
                else_branch: Some(Box::new(ElseBranch::ElseIf(ElseIf {
                    if_stmt: If {
                        expr: Expr::Boolean { b: false },
                        body: CodeBlock {
                            stmts: vec![Stmt::Expr(Expr::Number { n: 2.0 })]
                        },
                        else_branch: Some(Box::new(ElseBranch::Else(Else {
                            body: CodeBlock {
                                stmts: vec![Stmt::Expr(Expr::Number { n: 3.0 })]
                            }
                        })))
                    }
                })))
            })
        )
    }

    #[test]
    fn lower_else() {
        let root = parse("if true {1} else {3}");
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(
            hir,
            Stmt::If(If {
                expr: Expr::Boolean { b: true },
                body: CodeBlock {
                    stmts: vec![Stmt::Expr(Expr::Number { n: 1.0 })]
                },
                else_branch: Some(Box::new(ElseBranch::Else(Else {
                    body: CodeBlock {
                        stmts: vec![Stmt::Expr(Expr::Number { n: 3.0 })]
                    }
                })))
            })
        )
    }

    #[test]
    fn lower_list() {
        let root = parse("let x = [1,2,3]");
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(
            hir,
            Stmt::VariableDef(VariableDef {
                name: "x".into(),
                value: Expr::List {
                    items: vec![
                        Expr::Number { n: 1.0 },
                        Expr::Number { n: 2.0 },
                        Expr::Number { n: 3.0 },
                    ]
                }
            })
        )
    }

    #[test]
    fn lower_multi_type_list() {
        let root = parse(r#"let foo = [true, "bar", 1.2, 1]"#);
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(
            hir,
            Stmt::VariableDef(VariableDef {
                name: "foo".into(),
                value: Expr::List {
                    items: vec![
                        Expr::Boolean { b: true },
                        Expr::String { s: "bar".into() },
                        Expr::Number { n: 1.2 },
                        Expr::Number { n: 1.0 },
                    ]
                }
            })
        )
    }
}
