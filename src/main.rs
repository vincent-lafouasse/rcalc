#![allow(dead_code)]
#![allow(unused_mut)]

use std::vec;

enum NodeType {
    Number(i32),
    Add,
    Multiply,
}

struct Node {
    kind: NodeType,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
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
}

fn tokenize(input: String) -> Vec<NodeType> {
    let mut tokens: Vec<NodeType> = vec![];

    tokens
}

fn main() -> eyre::Result<()> {
    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input)?;

        println!("You typed: {}", input.trim());
    }
}
