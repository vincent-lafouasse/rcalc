#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::io::{stdin, stdout, Write};

mod lexer;

use lexer::{tokenize, Token};

fn main() -> eyre::Result<()> {
    loop {
        let mut input = String::new();
        print!("> ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;

        let tokens: Vec<Token> = tokenize(input);
        println!("{:?}", tokens);
    }
}
