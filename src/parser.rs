use crate::{
    errors::{LoxError, LoxErrorType},
    expr::Expr,
    literal::Literal,
    stmt::Stmt,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

type ParseResult<T> = Result<T, LoxError>;

macro_rules! match_any_token {
    ($parser:expr, $($token:expr),* ) => {
        $(
            Parser::match_token($parser, &$token) ||
        )* false
    };
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<LoxError>> {
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
        let result = match self.peek().token_type {
            TokenType::Fun => {
                self.advance();
                self.function_declaration("function")
            }
            TokenType::Var => {
                self.advance();
                self.var_declaration()
            }
            _ => self.statement(),
        };

        match result {
            Ok(stmt) => Ok(stmt),
            Err(reason) => {
                self.synchronize();
                Err(reason)
            }
        }
    }

    fn function_declaration(&mut self, kind: impl Into<String>) -> ParseResult<Stmt> {
        let kind = kind.into();
        let name = self.consume_identifier(format!("Expected {} name.", kind).as_str())?;
        self.consume(
            &TokenType::LeftParen,
            format!("Expected '(' after {} name.", kind).as_str(),
        )?;
        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                params.push(self.consume_identifier("Expected parameter name.")?);
                if params.len() >= 255 {
                    let err = LoxError::parse_error(
                        self.previous(),
                        format!("A {} cannot have more than 255 parameters.", kind),
                    );
                    return Err(err);
                }
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(&TokenType::RightParen, "Expected ')' after parameter list.")?;

        self.consume(
            &TokenType::LeftBrace,
            format!("Expected '{{' before {} body.", kind).as_str(),
        )?;

        let body = self.parse_block()?;

        Ok(Stmt::Function(name, params, body))
    }

    fn consume_identifier(&mut self, msg: &str) -> ParseResult<Token> {
        let token = self.peek();
        match token.token_type {
            TokenType::Identifier(_) => return Ok(self.advance().clone()),
            _ => {
                return Err(LoxError::parse_error(&token, msg.to_owned()));
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
            _ => {
                return Err(LoxError::parse_error(
                    &identifier,
                    "Expected variable name.",
                ))
            }
        }
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        match self.peek().token_type {
            TokenType::Print => {
                self.advance();
                self.print_statement()
            }
            TokenType::Return => {
                self.advance();
                self.return_statement()
            }
            TokenType::While => {
                self.advance();
                self.while_statement()
            }
            TokenType::For => {
                self.advance();
                self.for_statement()
            }
            TokenType::LeftBrace => {
                self.advance();
                let block = self.parse_block()?;
                Ok(Stmt::Block(block))
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

    fn return_statement(&mut self) -> ParseResult<Stmt> {
        let keyword = self.previous().clone();
        let value = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(&TokenType::Semicolon, "Expected ';' after return value.")?;
        Ok(Stmt::Return(keyword.clone(), value))
    }

    fn while_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(&TokenType::LeftParen, "Expected '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expected ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::While(condition, Box::new(body)))
    }

    fn for_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(&TokenType::LeftParen, "Expected '(' after 'for'.")?;
        let initializer = if self.match_token(&TokenType::Var) {
            Some(self.var_declaration()?)
        } else if self.match_token(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expr_statement()?)
        };

        let condition = match self.peek().token_type {
            TokenType::Semicolon => None,
            _ => Some(self.expression()?),
        };
        self.consume(&TokenType::Semicolon, "Expected ';' after loop condition.")?;

        let increment = match self.peek().token_type {
            TokenType::RightParen => None,
            _ => Some(self.expression()?),
        };
        self.consume(&TokenType::RightParen, "Expected ')' after for clause.")?;

        let body = self.statement()?;

        let body = match increment {
            Some(increment) => Stmt::Block(vec![body.clone(), Stmt::Expression(increment)]),
            None => body,
        };

        let condition = match condition {
            Some(condition) => condition,
            None => Expr::Literal(Literal::Boolean(true)),
        };

        let loop_stmt = Stmt::While(condition, Box::new(body.clone()));

        let result = match initializer {
            Some(initializer) => Stmt::Block(vec![initializer, loop_stmt]),
            None => body,
        };

        Ok(result)
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
                _ => {
                    return Err(LoxError::parse_error(
                        self.previous(),
                        "Invalid assignment target.".to_string(),
                    ))
                }
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

        while match_any_token!(self, TokenType::BangEqual, TokenType::EqualEqual) {
            let token = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), token.clone(), Box::new(right));
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.term()?;

        while match_any_token!(
            self,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
        }

        return Ok(expr);
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;

        while match_any_token!(self, TokenType::Minus, TokenType::Plus) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;

        while match_any_token!(self, TokenType::Slash, TokenType::Star) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator.to_owned(), Box::new(right));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        if match_any_token!(self, TokenType::Bang, TokenType::Minus) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator.to_owned(), Box::new(right)));
        }

        return self.call();
    }

    fn call(&mut self) -> ParseResult<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&TokenType::LeftParen) {
                expr = self.finish_call(&expr)?;
            } else {
                break;
            }
        }

        return Ok(expr);
    }

    fn finish_call(&mut self, callee: &Expr) -> ParseResult<Expr> {
        let mut args = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                args.push(self.expression()?);
                if args.len() >= 255 {
                    return Err(LoxError::parse_error(
                        self.previous(),
                        "Function call cannot have more than 255 arguments.".to_string(),
                    ));
                }
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expected ')' after argument list.")?;

        return Ok(Expr::Call(Box::new(callee.clone()), paren.clone(), args));
    }

    fn primary(&mut self) -> ParseResult<Expr> {
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
            _ => Err(LoxError::parse_error(
                self.previous(),
                "Expected expression".to_string(),
            )),
        }
    }

    fn consume(&mut self, token_type: &TokenType, msg: &str) -> ParseResult<&Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(LoxError::parse_error(&self.peek(), msg))
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
