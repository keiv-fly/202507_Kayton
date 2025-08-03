use super::*;

#[test]
fn program1_tokens() {
    let input = r#"
x = 12
x = x + 1
print(x)
"#;
    let tokens = Lexer::new(input).tokenize();
    assert_eq!(
        tokens,
        vec![
            Stmt::Assign {
                name: "x".to_string(),
                expr: Expr::Int(12),
            },
            Stmt::Assign {
                name: "x".to_string(),
                expr: Expr::Binary {
                    left: Box::new(Expr::Ident("x".to_string())),
                    op: BinOp::Add,
                    right: Box::new(Expr::Int(1)),
                },
            },
            Stmt::ExprStmt(Expr::Call {
                func: Box::new(Expr::Ident("print".to_string())),
                args: vec![Expr::Ident("x".to_string())],
            }),
        ]
    );
}

#[test]
fn program2_tokens() {
    let input = r#"print("Hello, World")"#;
    let tokens = Lexer::new(input).tokenize();
    assert_eq!(
        tokens,
        vec![Stmt::ExprStmt(Expr::Call {
            func: Box::new(Expr::Ident("print".to_string())),
            args: vec![Expr::Str("Hello, World".to_string())],
        }),]
    );
}

#[test]
fn program3_tokens() {
    let input = r#"
x = 12
print(f"{x}")
"#;
    let tokens = Lexer::new(input).tokenize();
    assert_eq!(
        tokens,
        vec![
            Stmt::Assign {
                name: "x".to_string(),
                expr: Expr::Int(12),
            },
            Stmt::ExprStmt(Expr::Call {
                func: Box::new(Expr::Ident("print".to_string())),
                args: vec![Expr::InterpolatedString(vec![
                    StringPart::Text("".to_string()),
                    StringPart::Expr(Box::new(Expr::Ident("x".to_string()))),
                    StringPart::Text("".to_string()),
                ])],
            }),
        ]
    );
}
