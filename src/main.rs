#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

mod lexer;

use lexer::{tokenize, Token};

fn main() -> eyre::Result<()> {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        println!("You typed: {}", input.trim());

        let tokens: Vec<Token> = tokenize(input);
    }
}
