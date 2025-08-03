use std::iter::Peekable;
use std::str::Chars;

// ===== Abstract syntax tree =====

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Assign { name: String, expr: Expr },
    ExprStmt(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Str(String),
    Ident(String),
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    InterpolatedString(Vec<StringPart>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringPart {
    Text(String),
    Expr(Box<Expr>),
}

// ===== Tokens for the lexer =====

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Int(i64),
    Str(String),
    Ident(String),
    Plus,
    Equal,
    LParen,
    RParen,
    Comma,
    Newline,
    EOF,
    InterpolatedString(Vec<StringPart>),
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    /// Tokenize the input into a list of statements.
    pub fn tokenize(mut self) -> Vec<Stmt> {
        let tokens = self.collect_tokens();
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }

    /// Collect raw tokens from the input. The resulting token stream will
    /// always be terminated with `Token::EOF`.
    fn collect_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let end = tok == Token::EOF;
            tokens.push(tok);
            if end {
                break;
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let ch = match self.chars.peek().copied() {
            Some(c) => c,
            None => return Token::EOF,
        };

        match ch {
            '\n' => {
                self.chars.next();
                Token::Newline
            }
            '=' => {
                self.chars.next();
                Token::Equal
            }
            '+' => {
                self.chars.next();
                Token::Plus
            }
            '(' => {
                self.chars.next();
                Token::LParen
            }
            ')' => {
                self.chars.next();
                Token::RParen
            }
            ',' => {
                self.chars.next();
                Token::Comma
            }
            '0'..='9' => self.lex_number(ch),
            'a'..='z' | 'A'..='Z' | '_' => {
                if ch == 'f' {
                    if let Some('"') = self.peek_next() {
                        return self.lex_fstring();
                    }
                }
                self.lex_ident(ch)
            }
            '"' => self.lex_string(),
            _ => {
                // Unknown character, skip
                self.chars.next();
                Token::EOF
            }
        }
    }

    fn lex_number(&mut self, first: char) -> Token {
        let mut num = first.to_string();
        self.chars.next();
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_digit() {
                num.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        Token::Int(num.parse().unwrap())
    }

    fn lex_ident(&mut self, first: char) -> Token {
        let mut ident = first.to_string();
        self.chars.next();
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() || *c == '_' {
                ident.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        Token::Ident(ident)
    }

    fn lex_string(&mut self) -> Token {
        self.chars.next(); // skip opening quote
        let mut s = String::new();
        while let Some(c) = self.chars.next() {
            if c == '"' {
                break;
            } else {
                s.push(c);
            }
        }
        Token::Str(s)
    }

    fn lex_fstring(&mut self) -> Token {
        self.chars.next(); // consume 'f'
        self.chars.next(); // consume opening quote
        let mut parts = vec![StringPart::Text(String::new())];
        let mut current_index = 0; // index of current text part
        while let Some(c) = self.chars.next() {
            match c {
                '"' => break,
                '{' => {
                    // collect expression between braces and parse it
                    let mut expr_src = String::new();
                    while let Some(ch) = self.chars.next() {
                        if ch == '}' {
                            break;
                        } else {
                            expr_src.push(ch);
                        }
                    }
                    let expr = parse_embedded_expr(&expr_src);
                    parts.push(StringPart::Expr(Box::new(expr)));
                    parts.push(StringPart::Text(String::new()));
                    current_index = parts.len() - 1;
                }
                _ => {
                    if let StringPart::Text(ref mut t) = parts[current_index] {
                        t.push(c);
                    }
                }
            }
        }
        Token::InterpolatedString(parts)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c == ' ' || c == '\t' || c == '\r' {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn peek_next(&mut self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();
        iter.peek().copied()
    }
}

// ===== Parser implementation =====

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !self.is_at_end() {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
            self.skip_newlines();
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        if self.is_at_end() {
            return None;
        }
        if let Token::Ident(name) = self.peek().clone() {
            if self.peek_next_is(Token::Equal) {
                self.advance(); // ident
                self.advance(); // '='
                let expr = self.parse_expr();
                return Some(Stmt::Assign { name, expr });
            }
        }
        let expr = self.parse_expr();
        Some(Stmt::ExprStmt(expr))
    }

    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_primary();
        while matches!(self.peek(), Token::Plus) {
            self.advance();
            let right = self.parse_primary();
            left = Expr::Binary {
                left: Box::new(left),
                op: BinOp::Add,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Token::Int(n) => Expr::Int(n),
            Token::Str(s) => Expr::Str(s),
            Token::Ident(s) => {
                let expr = Expr::Ident(s);
                self.parse_call(expr)
            }
            Token::InterpolatedString(parts) => Expr::InterpolatedString(parts),
            Token::LParen => {
                let expr = self.parse_expr();
                self.expect(Token::RParen);
                self.parse_call(expr)
            }
            other => panic!("Unexpected token {:?}", other),
        }
    }

    fn parse_call(&mut self, mut expr: Expr) -> Expr {
        loop {
            match self.peek() {
                Token::LParen => {
                    self.advance(); // consume '(' 
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Token::RParen) {
                        args.push(self.parse_expr());
                        while matches!(self.peek(), Token::Comma) {
                            self.advance();
                            args.push(self.parse_expr());
                        }
                    }
                    self.expect(Token::RParen);
                    expr = Expr::Call {
                        func: Box::new(expr),
                        args,
                    };
                }
                _ => break,
            }
        }
        expr
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline) {
            self.advance();
        }
    }

    fn expect(&mut self, expected: Token) {
        let tok = self.advance();
        if tok != expected {
            panic!("expected {:?}, found {:?}", expected, tok);
        }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.pos).cloned().unwrap_or(Token::EOF)
    }

    fn peek_next_is(&self, expected: Token) -> bool {
        self.tokens
            .get(self.pos + 1)
            .cloned()
            .map_or(false, |t| t == expected)
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek();
        if !self.is_at_end() {
            self.pos += 1;
        }
        tok
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::EOF)
    }
}

fn parse_embedded_expr(src: &str) -> Expr {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.collect_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse_expr()
}

mod tests;
