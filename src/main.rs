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

fn tokenize(input: String) -> Vec<NodeType>
{
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
