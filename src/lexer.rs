pub enum Token {
    Number(i32),
    Identifier(String),
    Add,
    Multiply,
    Equal,
    EOF,
}

pub struct Lexer {
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

    fn scan_next_token(&mut self) -> Option<Token> {
        let c = match self.advance() {
            None => return None,
            Some(c) => c,
        };

        match c {
            b'+' => Some(Token::Add),
            b'*' => Some(Token::Multiply),
            b'=' => Some(Token::Equal),
            _ => None,
        }

    }

    fn advance(&mut self) -> Option<u8> {
        match !self.is_at_end() {
            true => {
                let out = self.chars[self.current];
                self.current += 1;
                Some(out)
            }
            false => None
        }
    }

    fn is_at_end(&self) -> bool {
        self.start >= self.chars.len()
    }
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = vec![];

    while let Some(token) = lexer.scan_next_token()
    {
        tokens.push(token);
    }
    tokens.push(Token::EOF);

    tokens
}
