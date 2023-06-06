use crate::{
    environment::Environment,
    errors::DetailedErrorType,
    errors::LoxError,
    errors::LoxErrorType,
    expr::{Expr, Stmt},
    literal::Literal,
    token::{Token, TokenType},
};

pub type EvaluationResult = Result<Literal, LoxError>;

fn evaluate_arithmetic(operator: Token, left: Literal, right: Literal) -> EvaluationResult {
    match (left, right) {
        (Literal::Number(left), Literal::Number(right)) => match operator.token_type {
            TokenType::Plus => Ok(Literal::Number(left + right)),
            TokenType::Minus => Ok(Literal::Number(left - right)),
            TokenType::Slash => Ok(Literal::Number(left / right)),
            TokenType::Star => Ok(Literal::Number(left * right)),
            _ => panic!(),
        },

        _ => Err(LoxError::new(
            &operator,
            LoxErrorType::RuntimeError,
            DetailedErrorType::ExpectedNumber,
        )),
    }
}

fn evaluate_comparison(operator: Token, left: Literal, right: Literal) -> EvaluationResult {
    match (left, right) {
        (Literal::Number(left), Literal::Number(right)) => match operator.token_type {
            TokenType::Less => Ok(Literal::Boolean(left < right)),
            TokenType::LessEqual => Ok(Literal::Boolean(left <= right)),
            TokenType::Greater => Ok(Literal::Boolean(left > right)),
            TokenType::GreaterEqual => Ok(Literal::Boolean(left >= right)),
            _ => panic!(),
        },

        _ => Err(LoxError::new(
            &operator,
            LoxErrorType::RuntimeError,
            DetailedErrorType::ExpectedNumber,
        )),
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn evaluate_statement(&mut self, stmt: Stmt) -> EvaluationResult {
        match stmt {
            Stmt::Print(expr) => self.evaluate_print(expr),
            Stmt::Expression(expr) => self.evaluate(expr),
            Stmt::Var(identifier, Some(initializer)) => match self.evaluate(initializer) {
                Ok(value) => {
                    self.environment.define(&identifier.lexeme, value);
                    Ok(Literal::Nil)
                }
                Err(reason) => Err(reason),
            },
            Stmt::Var(identifier, None) => {
                self.environment.define(&identifier.lexeme, Literal::Nil);
                Ok(Literal::Nil)
            }
        }
    }

    fn evaluate_print(&mut self, expr: Expr) -> EvaluationResult {
        match self.evaluate(expr) {
            Ok(value) => {
                println!("{}", value);
                Ok(Literal::Nil)
            }
            other => other,
        }
    }

    pub fn evaluate(&mut self, expr: Expr) -> EvaluationResult {
        match expr {
            Expr::Literal(value) => Ok(value.to_owned()),
            Expr::Grouping(expr) => self.evaluate(*expr),
            Expr::Unary(operator, right) => self.evaluate_unary_expression(operator, right),
            Expr::Binary(left, operator, right) => {
                self.evaluate_binary_expression(left, operator, right)
            }
            Expr::Var(identifier) => match self.environment.fetch(&identifier.lexeme) {
                Some(value) => Ok(value.to_owned()),
                None => Err(LoxError::new(
                    &identifier,
                    LoxErrorType::RuntimeError,
                    DetailedErrorType::UndeclaredIdentifier,
                )),
            },
        }
    }

    fn evaluate_unary_expression(&mut self, operator: Token, right: Box<Expr>) -> EvaluationResult {
        let right = self.evaluate(*right);
        if right.is_err() {
            return right;
        }

        let right = right.unwrap();
        match operator.token_type {
            TokenType::Minus => match right {
                Literal::Number(value) => Ok(Literal::Number(-value)),
                _ => Err(LoxError::new(
                    &operator,
                    LoxErrorType::RuntimeError,
                    DetailedErrorType::ExpectedNumber,
                )),
            },
            TokenType::Bang => return Ok(Literal::Boolean(right.is_truthy())),
            _ => {
                panic!()
            }
        }
    }

    fn evaluate_binary_expression(
        &mut self,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    ) -> EvaluationResult {
        let left = self.evaluate(*left);
        if left.is_err() {
            return left;
        }
        let right = self.evaluate(*right);
        if right.is_err() {
            return right;
        }

        let left = left.unwrap();
        let right = right.unwrap();

        match operator.token_type {
            TokenType::Plus => match (&left, &right) {
                (Literal::String(left), Literal::String(right)) => {
                    let concatenated = format!("{}{}", left, right);
                    return Ok(Literal::String(concatenated));
                }
                _ => evaluate_arithmetic(operator, left, right),
            },
            TokenType::Minus | TokenType::Star | TokenType::Slash => {
                evaluate_arithmetic(operator, left, right)
            }
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => evaluate_comparison(operator, left, right),
            TokenType::EqualEqual => Ok(Literal::Boolean(left == right)),
            TokenType::BangEqual => Ok(Literal::Boolean(left != right)),
            _ => panic!(),
        }
    }
}
