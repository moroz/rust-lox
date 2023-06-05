use crate::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub struct ScanError {
    pub line: usize,
    pub message: String,
    pub lexeme: Option<String>,
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

fn is_digit(c: &char) -> bool {
    ('0'..='9').contains(c)
}

fn is_alpha(c: &char) -> bool {
    ('a'..='z').contains(c) || ('A'..='Z').contains(c) || c == &'_'
}

fn is_alphanumeric(c: &char) -> bool {
    is_digit(c) || is_alpha(c)
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

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 > self.final_index {
            return None;
        }
        return self.source.get(self.current + 1).cloned();
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.get_current_lexeme();
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

            '0'..='9' => self.scan_number(),

            'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier(),

            '\n' => self.line += 1,

            other => self.errors.push(ScanError {
                line: self.line,
                message: "Unexpected character.".to_string(),
                lexeme: Some(other.to_string()),
            }),
        }
    }

    fn add_error(&mut self, message: String, lexeme: Option<String>) {
        self.errors.push(ScanError {
            line: self.line,
            message,
            lexeme,
        })
    }

    fn scan_string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.add_error("Unterminated string literal".to_string(), None);
            return;
        }

        self.advance();

        let range = (self.start + 1)..(self.current - 1);
        let value: String = self.source[range].iter().collect();
        self.add_token(TokenType::String(value));
    }

    fn get_current_lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    fn scan_number(&mut self) {
        while let Some(digit) = self.peek() {
            if is_digit(&digit) {
                self.advance();
            } else {
                break;
            }
        }

        if self.peek() == Some('.') {
            if let Some(digit) = self.peek_next() {
                if is_digit(&digit) {
                    self.advance();
                    while let Some(digit) = self.peek() {
                        if is_digit(&digit) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        let as_string = self.get_current_lexeme();
        let value: f64 = as_string.parse().unwrap();
        self.add_token(TokenType::Number(value));
    }

    fn scan_identifier(&mut self) {
        while let Some(c) = self.peek() {
            if is_alphanumeric(&c) {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = self.get_current_lexeme();
        self.add_token(Token::match_keyword(lexeme.as_str()));
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
