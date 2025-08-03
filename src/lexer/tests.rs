use super::*;

#[test]
fn tokenize_assignment() {
    let input = "x = 12\n";
    let tokens = Lexer::new(input).tokenize();
    assert_eq!(
        tokens,
        vec![
            Token::Ident("x".to_string()),
            Token::Equal,
            Token::Int(12),
            Token::Newline,
            Token::EOF,
        ]
    );
}

#[test]
fn tokenize_print_call() {
    let input = r#"print("hi")"#;
    let tokens = Lexer::new(input).tokenize();
    assert_eq!(
        tokens,
        vec![
            Token::Ident("print".to_string()),
            Token::LParen,
            Token::Str("hi".to_string()),
            Token::RParen,
            Token::EOF,
        ]
    );
}

#[test]
fn tokenize_fstring() {
    let input = r#"print(f"{x}")"#;
    let tokens = Lexer::new(input).tokenize();
    assert_eq!(
        tokens,
        vec![
            Token::Ident("print".to_string()),
            Token::LParen,
            Token::InterpolatedString(vec![
                StringPart::Text("".to_string()),
                StringPart::Expr(vec![Token::Ident("x".to_string())]),
                StringPart::Text("".to_string()),
            ]),
            Token::RParen,
            Token::EOF,
        ]
    );
}
