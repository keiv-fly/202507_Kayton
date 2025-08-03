use super::*;

#[test]
fn program1_tokens() {
    let input = "x = 12\nx = x + 1\nprint(x)\n";
    let tokens = Lexer::new(input).tokenize();
    assert_eq!(tokens, vec![
        Token::Ident("x".into()), Token::Equal, Token::Int(12), Token::Newline,
        Token::Ident("x".into()), Token::Equal, Token::Ident("x".into()), Token::Plus, Token::Int(1), Token::Newline,
        Token::Ident("print".into()), Token::LParen, Token::Ident("x".into()), Token::RParen, Token::Newline,
        Token::EOF,
    ]);
}

#[test]
fn program2_tokens() {
    let input = "print(\"Hello, World\")\n";
    let tokens = Lexer::new(input).tokenize();
    assert_eq!(tokens, vec![
        Token::Ident("print".into()), Token::LParen, Token::Str("Hello, World".into()), Token::RParen, Token::Newline, Token::EOF,
    ]);
}

#[test]
fn program3_tokens() {
    let input = "x = 12\nprint(f\"{x}\")\n";
    let tokens = Lexer::new(input).tokenize();
    let parts = vec![
        StringPart::Text("".into()),
        StringPart::Expr(vec![Token::Ident("x".into())]),
        StringPart::Text("".into()),
    ];
    assert_eq!(tokens, vec![
        Token::Ident("x".into()), Token::Equal, Token::Int(12), Token::Newline,
        Token::Ident("print".into()), Token::LParen, Token::InterpolatedString(parts), Token::RParen, Token::Newline,
        Token::EOF,
    ]);
}
