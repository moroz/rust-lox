use crate::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub struct ScanError {
    pub line: usize,
    pub message: String,
    pub lexeme: String,
}

pub struct ScanResult {
    pub tokens: Vec<Token>,
    pub errors: Vec<ScanError>,
}

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    final_index: usize,
    tokens: Vec<Token>,
    errors: Vec<ScanError>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
            start: 0,
            line: 1,
            final_index: source.chars().count(),
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        let returned = self.source.get(self.current).cloned();
        self.current += 1;
        returned
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.final_index
    }

    fn match_lookahead(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let next_char = self.source.get(self.current).cloned().unwrap();
        if next_char != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        return self.source.get(self.current).cloned();
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let token = Token::new(token_type, lexeme, self.line);
        self.tokens.push(token);
    }

    fn scan_token(&mut self) {
        let next_char = self.advance().unwrap();
        match next_char {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_lookahead('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_lookahead('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_lookahead('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_lookahead('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }

            '/' => {
                // When you find a comment, skip to the end of the line
                if self.match_lookahead('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            // Ignore whitespace
            ' ' | '\r' | '\t' => (),

            '"' => self.scan_string(),

            '\n' => self.line += 1,

            other => self.errors.push(ScanError {
                line: self.line,
                message: "Unexpected character.".to_string(),
                lexeme: other.to_string(),
            }),
        }
    }

    fn scan_string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }
    }

    pub fn scan_tokens(&mut self) -> Result<ScanResult, ScanResult> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let result = ScanResult {
            errors: self.errors.clone(),
            tokens: self.tokens.clone(),
        };
        if self.errors.is_empty() {
            return Ok(result);
        } else {
            return Err(result);
        }
    }
}
