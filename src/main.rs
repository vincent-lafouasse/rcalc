enum NodeType
{
    Number(i32),
    Add,
    Multiply
}

struct Node {
    kind: NodeType,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}


fn main() -> eyre::Result<()> {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;

    println!("You typed: {}", input.trim());
    Ok(())
}
