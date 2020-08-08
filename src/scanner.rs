use std::str::Chars;

pub struct Scanner<'a> {
    pub source: &'a str,
    // Start of current item.
    pub start: usize,
    // current position in source.
    pub pos: usize,
    pub line: u32,
    pub width: usize,
}

#[derive(Copy, Clone)]
pub struct Token<'a> {
    pub t_type: TokenType,
    pub lexeme: &'a str,
    // line: u32,
}

#[derive(Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Template strings
    TemplateStart,
    TemplateEnd,
    TemplatePart, // Part of literal text inside a template
    InterpolationStart,
    InterpolationEnd,

    // Literals.
    Identifier,
    TokenString,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

impl<'a> Scanner<'a> {
    // TODO: maybe make scanning more state-machine like.
    pub fn scan_token(&mut self) -> Token<'a> {
        self.start = self.pos;
        self.skip_whitespace();
        if let Some(c) = self.advance() {
            if c.is_digit(10) {
                return self.number();
            }
            if c.is_alphabetic() {
                return self.identifier();
            }

            match c {
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '{' => self.make_token(TokenType::LeftBrace),
                '}' => self.make_token(TokenType::RightBrace),
                ';' => self.make_token(TokenType::Semicolon),
                ',' => self.make_token(TokenType::Comma),
                '.' => self.make_token(TokenType::Dot),
                '-' => self.make_token(TokenType::Minus),
                '+' => self.make_token(TokenType::Plus),
                '/' => self.make_token(TokenType::Slash),
                '*' => self.make_token(TokenType::Star),
                '!' => {
                    if self.accept('=') {
                        self.make_token(TokenType::BangEqual)
                    } else {
                        self.make_token(TokenType::Bang)
                    }
                }
                '=' => {
                    if self.accept('=') {
                        self.make_token(TokenType::EqualEqual)
                    } else {
                        self.make_token(TokenType::Equal)
                    }
                }
                '<' => {
                    if self.accept('=') {
                        self.make_token(TokenType::LessEqual)
                    } else {
                        self.make_token(TokenType::Less)
                    }
                }
                '>' => {
                    if self.accept('=') {
                        self.make_token(TokenType::GreaterEqual)
                    } else {
                        self.make_token(TokenType::Greater)
                    }
                }
                '$' => {
                    if self.accept('"') {
                        self.templatestring()
                    } else {
                        error_token("Dollar sign is not valid in literals")
                    }
                }
                '"' => self.string(),
                _ => error_token("Unexpected character."),
            }
        } else {
            self.make_token(TokenType::Eof)
        }
    }

    fn advance(&mut self) -> Option<char> {
        // Not sure if its a good idea to create this iterator just to get one char.
        let rest = self.source.get(self.pos..)?;
        let next = rest.chars().next()?;
        self.width = next.len_utf8();
        self.pos += self.width;
        Some(next)
    }

    fn backup(&mut self) {
        if self.pos >= self.width {
            self.pos -= self.width;
        }
    }

    // Ignore current item, move past it.
    fn ignore(&mut self) {
        self.start = self.pos;
    }

    fn peek(&mut self) -> Option<char> {
        let next = self.advance();
        self.backup();
        next
    }

    fn peek_many(&mut self, count: usize) -> Option<char> {
        for i in 0..count - 1 {
            if let Some(c) = self.advance() {}
        }

        let next = self.advance();
        for i in 0..count {
            self.backup()
        }
        next
    }

    // Consume an expected character or backup.
    fn accept(&mut self, expected: char) -> bool {
        if let Some(c) = self.advance() {
            if c == expected {
                return true;
            } else {
                self.backup();
            }
        }
        false
    }

    // Consume an expected character with predicate.
    fn accept_with(&mut self, predicate: fn(c: char) -> bool) -> bool {
        if let Some(c) = self.advance() {
            if predicate(c) {
                return true;
            } else {
                self.backup();
            }
        }
        false
    }

    // Accept many, return the count of accepted chars.
    fn accept_with_many(&mut self, predicate: fn(c: char) -> bool) -> usize {
        let mut count = 0;
        while self.accept_with(predicate) {
            count += 1;
        }
        return count;
    }

    fn skip_whitespace(&mut self) {
        loop {
            if let Some(c) = self.advance() {
                match c {
                    ' ' | '\r' | '\t' => {}
                    '\n' => self.line += 1,
                    // Line comments.
                    '/' => {
                        if self.accept('/') {
                            // Consume until endline.
                            while self.accept_with(|c| c != '\n') {}
                        } else {
                            self.backup()
                        }
                    }
                    _ => {
                        self.backup();
                        return;
                    }
                }
            }
        }
    }

    fn string(&mut self) -> Token<'a> {
        loop {
            if let Some(c) = self.advance() {
                match c {
                    '"' => break, // String ended.
                    '\n' => self.line += 1,
                    _ => {}
                }
                continue;
            }
            return error_token("Unterminated string.");
        }

        self.make_token(TokenType::TokenString)
    }

    fn templatestring(&mut self) -> Token<'a> {
        panic!("templatestring not implemented")
    }

    fn identifier(&mut self) -> Token<'a> {
        self.accept_with_many(|c| c.is_alphanumeric());
        let t_type = self.identifier_type();
        self.make_token(t_type)
    }

    fn identifier_type(&mut self) -> TokenType {
        match &self.source[self.start..=self.pos] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }

    fn number(&mut self) -> Token<'a> {
        self.accept_with_many(is_digit);

        // Look for fractional part.
        if self.accept('.') {
            if self.accept_with_many(is_digit) == 0 {
                // There was not digits after the dot, don't consume it.
                self.backup();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn make_token(&mut self, t_type: TokenType) -> Token<'a> {
        Token {
            t_type: t_type,
            lexeme: &self.source[self.start..=self.pos],
        }
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn error_token(message: &str) -> Token {
    Token {
        t_type: TokenType::Error,
        lexeme: message,
        // line: scanner.line,
    }
}

// impl Scanner {
//     // pub fn new(source: &String) -> Scanner {

//     // }
// }
