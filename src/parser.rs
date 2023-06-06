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
pub struct ParseError(String);

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<ParseError>> {
        if self.tokens.len() == 1 {
            return Ok(vec![Stmt::Expression(Expr::Literal(Literal::Nil))]);
        }

        let mut program = Vec::new();
        let mut errors = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => {
                    program.push(stmt);
                }
                Err(reason) => {
                    errors.push(reason);
                    self.synchronize();
                }
            }
        }
        if errors.is_empty() {
            return Ok(program);
        }
        return Err(errors);
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
                    initializer = Some(self.expression()?);
                }
                self.consume(
                    &TokenType::Semicolon,
                    "Expected ';' after variable declaration",
                )?;
                return Ok(Stmt::Var(identifier, initializer));
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
            TokenType::If => {
                self.advance();
                self.parse_if()
            }
            _ => self.expr_statement(),
        }
    }

    fn parse_if(&mut self) -> ParseResult<Stmt> {
        self.consume(&TokenType::LeftParen, "Expected '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expected ')' after if condition.")?;
        let then_branch = self.statement()?;
        if self.match_token(&TokenType::Else) {
            let else_branch = self.statement()?;
            return Ok(Stmt::If(
                condition,
                Box::new(then_branch),
                Some(Box::new(else_branch)),
            ));
        }
        return Ok(Stmt::If(condition, Box::new(then_branch), None));
    }

    fn parse_block(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }

        self.consume(&TokenType::RightBrace, "Expected '}' after block.")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected semicolon")?;
        Ok(Stmt::Print(expr))
    }

    fn expr_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected semicolon")?;
        Ok(Stmt::Expression(expr))
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
        let expr = self.or();

        if self.match_token(&TokenType::Equal) {
            let value = self.assignment()?;

            match expr {
                Ok(Expr::Var(name)) => {
                    return Ok(Expr::Assign(name, Box::new(value)));
                }
                _ => return Err(ParseError("Invalid assignment target.".to_string())),
            }
        }

        return expr;
    }

    fn or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.and()?;

        while self.match_token(&TokenType::Or) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.equality()?;

        while self.match_token(&TokenType::And) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;

        let token_types = vec![TokenType::BangEqual, TokenType::EqualEqual];
        while self.match_any_token(&token_types) {
            let token = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), token.clone(), Box::new(right));
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.term()?;

        let token_types = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
        }

        return Ok(expr);
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;

        let token_types = vec![TokenType::Minus, TokenType::Plus];
        while self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;

        let token_types = vec![TokenType::Slash, TokenType::Star];
        while self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        let token_types = vec![TokenType::Bang, TokenType::Minus];
        if self.match_any_token(&token_types) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator.to_owned(), Box::new(right)));
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
