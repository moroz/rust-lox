use crate::{
    expr::{Expr, Stmt},
    literal::Literal,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Clone, Debug)]
struct ParseError(String);

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        if self.tokens.len() == 1 {
            return vec![Stmt::Expression(Expr::Literal(Literal::Nil))];
        }

        let mut program = Vec::new();
        while !self.is_at_end() {
            if let Ok(stmt) = self.declaration() {
                program.push(stmt);
            } else {
                self.synchronize();
            }
        }
        return program;
    }

    fn declaration(&mut self) -> ParseResult<Stmt> {
        let result = if self.match_token(&TokenType::Var) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match result {
            Ok(stmt) => Ok(stmt),
            Err(reason) => {
                self.synchronize();
                Err(reason)
            }
        }
    }

    fn var_declaration(&mut self) -> ParseResult<Stmt> {
        let identifier = self.peek();
        match identifier.token_type {
            TokenType::Identifier(_) => {
                self.advance();

                let mut initializer = None;
                if self.match_token(&TokenType::Equal) {
                    match self.expression() {
                        Err(reason) => {
                            return Err(reason);
                        }
                        Ok(expr) => {
                            initializer = Some(expr);
                        }
                    }
                }
                match self.consume(
                    &TokenType::Semicolon,
                    "Expected ';' after variable declaration",
                ) {
                    Ok(_) => {
                        return Ok(Stmt::Var(identifier, initializer));
                    }
                    Err(reason) => Err(reason),
                }
            }
            _ => return Err(ParseError("Expected variable name.".to_string())),
        }
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        match self.peek().token_type {
            TokenType::Print => {
                self.advance();
                self.print_statement()
            }
            TokenType::LeftBrace => {
                self.advance();
                self.parse_block().map(|block| Stmt::Block(block))
            }
            _ => self.expr_statement(),
        }
    }

    fn parse_block(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(reason) => {
                    return Err(reason);
                }
            }
        }

        match self.consume(&TokenType::RightBrace, "Expected '}' after block.") {
            Ok(_) => Ok(statements),
            Err(reason) => Err(reason),
        }
    }

    fn print_statement(&mut self) -> ParseResult<Stmt> {
        match self.expression() {
            Err(reason) => Err(reason),
            Ok(expr) => match self.consume(&TokenType::Semicolon, "Expected semicolon") {
                Ok(_) => Ok(Stmt::Print(expr)),
                Err(reason) => Err(reason),
            },
        }
    }

    fn expr_statement(&mut self) -> ParseResult<Stmt> {
        match self.expression() {
            Err(reason) => {
                return Err(reason);
            }
            Ok(expr) => match self.consume(&TokenType::Semicolon, "Expected semicolon") {
                Ok(_) => Ok(Stmt::Expression(expr)),
                Err(reason) => Err(reason),
            },
        }
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

    fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.equality();
        if expr.is_err() {
            return expr;
        }

        if self.match_token(&TokenType::Equal) {
            let value = self.assignment();
            if value.is_err() {
                return value;
            }

            match expr {
                Ok(Expr::Var(name)) => {
                    return Ok(Expr::Assign(name, Box::new(value.unwrap())));
                }
                _ => return Err(ParseError("Invalid assignment target.".to_string())),
            }
        }

        return expr;
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let expr = self.comparison();
        if expr.is_err() {
            return expr;
        }
        let mut expr = expr.unwrap();

        let token_types = vec![TokenType::BangEqual, TokenType::EqualEqual];
        while self.match_any_token(&token_types) {
            let token = self.previous().clone();
            match self.comparison() {
                Ok(right) => {
                    expr = Expr::Binary(Box::new(expr), token.clone(), Box::new(right));
                }
                Err(reason) => {
                    return Err(reason);
                }
            }
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let expr = self.term();
        if expr.is_err() {
            return expr;
        }
        let mut expr = expr.unwrap();

        let token_types = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            match self.term() {
                Ok(right) => {
                    expr = Expr::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
                }
                Err(reason) => {
                    return Err(reason);
                }
            }
        }

        return Ok(expr);
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let expr = self.factor();
        if expr.is_err() {
            return expr;
        }
        let mut expr = expr.unwrap();

        let token_types = vec![TokenType::Minus, TokenType::Plus];
        while self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            match self.factor() {
                Ok(right) => {
                    expr = Expr::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
                }
                Err(reason) => {
                    return Err(reason);
                }
            }
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let expr = self.unary();
        if expr.is_err() {
            return expr;
        }
        let mut expr = expr.unwrap();

        let token_types = vec![TokenType::Slash, TokenType::Star];
        while self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            match self.unary() {
                Ok(right) => {
                    expr = Expr::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
                }
                Err(reason) => return Err(reason),
            }
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        let token_types = vec![TokenType::Bang, TokenType::Minus];
        if self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            let right = self
                .unary()
                .map(|right| Expr::Unary(operator.to_owned(), Box::new(right)));
            return right;
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().token_type {
            TokenType::False => {
                self.advance();
                return Ok(Expr::Literal(Literal::Boolean(false)));
            }
            TokenType::True => {
                self.advance();
                return Ok(Expr::Literal(Literal::Boolean(true)));
            }
            TokenType::Nil => {
                self.advance();
                return Ok(Expr::Literal(Literal::Nil));
            }
            TokenType::Number(value) => {
                self.advance();
                return Ok(Expr::Literal(Literal::Number(value)));
            }
            TokenType::String(value) => {
                self.advance();
                return Ok(Expr::Literal(Literal::String(value.clone())));
            }
            TokenType::LeftParen => {
                self.advance();
                match self.expression() {
                    Err(reason) => Err(reason),
                    Ok(expr) => {
                        match self.consume(&TokenType::RightParen, "Expected ')' after expression.")
                        {
                            Ok(_) => Ok(Expr::Grouping(Box::new(expr))),
                            Err(reason) => Err(reason),
                        }
                    }
                }
            }
            TokenType::Identifier(_) => {
                return Ok(Expr::Var(self.advance().to_owned()));
            }
            _ => Err(ParseError("Expected expression".to_string())),
        }
    }

    fn consume(&mut self, token_type: &TokenType, msg: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(ParseError(msg.to_owned()))
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
