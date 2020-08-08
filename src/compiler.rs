use crate::chunk::Chunk;
use crate::scanner::Scanner;
use crate::scanner::Token;
use crate::scanner::TokenType;

pub struct Parser<'a> {
    current: Option<Token<'a>>,
    previous: Option<Token<'a>>,
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            let token = self.scanner.scan_token();
            self.current = Some(token);
            match token.t_type {
                TokenType::Error => self.error_at_current(),
                _ => break,
            }
        }
    }

    fn error_at_current(&mut self) {
        println!("Error from parser")
    }
}

pub fn compile(source: &str, chunk: &Chunk) {
    let mut parser = Parser {
        current: None,
        previous: None,
        scanner: Scanner {
            source: source,
            start: 0,
            pos: 0,
            line: 1,
            width: 1,
        },
    };

    parser.advance();
}
