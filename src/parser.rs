use core::panic;

use crate::{
    expr::{Expression, Literal},
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expression> {
        Some(self.expression())
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => (),
            }

            self.advance();
        }
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        let token_types = vec![TokenType::BangEqual, TokenType::EqualEqual];
        while self.match_any_token(&token_types) {
            let token = self.previous().clone();
            let right = self.comparison();
            expr = Expression::Binary(Box::new(expr), token.clone(), Box::new(right));
        }

        return expr;
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        let token_types = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expression::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
        }

        return expr;
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        let token_types = vec![TokenType::Minus, TokenType::Plus];
        while self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expression::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
        }

        return expr;
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        let token_types = vec![TokenType::Slash, TokenType::Star];
        while self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expression::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
        }

        return expr;
    }

    fn unary(&mut self) -> Expression {
        let token_types = vec![TokenType::Bang, TokenType::Minus];
        if self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expression::Unary(operator.to_owned(), Box::new(right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expression {
        match self.peek().token_type {
            TokenType::False => {
                self.advance();
                return Expression::Literal(Literal::Boolean(false));
            }
            TokenType::True => {
                self.advance();
                return Expression::Literal(Literal::Boolean(true));
            }
            TokenType::Nil => {
                self.advance();
                return Expression::Literal(Literal::Nil);
            }
            TokenType::Number(value) => {
                self.advance();
                return Expression::Literal(Literal::Number(value));
            }
            TokenType::String(value) => {
                self.advance();
                return Expression::Literal(Literal::String(value.clone()));
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(&TokenType::RightParen, "Expected ')' after expression.");
                return Expression::Grouping(Box::new(expr));
            }
            _ => {
                panic!("Expected expression")
            }
        }
    }

    fn consume(&mut self, token_type: &TokenType, msg: &str) -> &Token {
        if self.check(token_type) {
            return self.advance();
        }

        panic!("{}", msg);
    }

    fn match_any_token(&mut self, tokens: &Vec<TokenType>) -> bool {
        for token in tokens {
            if self.match_token(token) {
                return true;
            }
        }
        return false;
    }

    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }

        return false;
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn is_at_end(&mut self) -> bool {
        match self.peek().token_type {
            TokenType::EOF => true,
            _ => false,
        }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().to_owned()
    }
}
