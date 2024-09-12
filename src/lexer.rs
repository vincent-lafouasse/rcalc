#[derive(Debug)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Add,
    Multiply,
    Equal,
    Eof,
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = vec![];

    while let Some(token) = lexer.scan_next_token() {
        tokens.push(token);
    }
    tokens.push(Token::Eof);

    tokens
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
        self.skip_whitespace();
        let c = match self.advance() {
            None => return None,
            Some(c) => c,
        };

        if let Some(token) = match c {
            b'+' => Some(Token::Add),
            b'*' => Some(Token::Multiply),
            b'=' => Some(Token::Equal),
            (b'0'..=b'9') => None,
            _ => None,
        } {
            return Some(token);
        }

        None
    }

    fn scan_number(&mut self) -> Option<f64> {
        Some(0.0)
    }

    fn advance(&mut self) -> Option<u8> {
        let out = self.peek();
        self.current += 1;
        out
    }

    fn peek(&self) -> Option<u8> {
        match !self.is_at_end() {
            true => Some(self.chars[self.current]),
            false => None,
        }
    }

    fn peek_next(&self) -> Option<u8> {
        if self.current + 1 >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.current + 1])
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.start >= self.chars.len()
    }
}
