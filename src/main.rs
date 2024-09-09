#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::vec;

enum Token {
    Number(i32),
    Add,
    Multiply,
}

struct Lexer {
    chars: Vec<u8>,
    start: usize,
    current: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        let chars: Vec<u8> = input.as_bytes().to_vec();
        Self {
            chars,
            start: 0,
            current: 0,
        }
    }

    fn scan_next_token(&self) -> Option<Token> {
        Some(Token::Multiply)
    }
}

fn tokenize(input: String) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = vec![];

    tokens
}

fn main() -> eyre::Result<()> {
    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input)?;

        println!("You typed: {}", input.trim());
    }
}
