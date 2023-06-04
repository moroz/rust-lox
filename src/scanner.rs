use std::str::Chars;

use crate::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    final_index: usize,
    tokens: Vec<Token>,
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
            _ => (),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        unimplemented!()
    }
}
