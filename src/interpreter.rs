use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    environment::Environment,
    errors::DetailedErrorType,
    errors::LoxError,
    errors::LoxErrorType,
    expr::Expr,
    literal::Literal,
    stmt::Stmt,
    token::{Token, TokenType},
};

pub type EvaluationResult = Result<Literal, LoxError>;

fn evaluate_arithmetic(operator: &Token, left: &Literal, right: &Literal) -> EvaluationResult {
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

fn evaluate_comparison(operator: &Token, left: &Literal, right: &Literal) -> EvaluationResult {
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
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        Self {
            globals: Rc::clone(&globals),
            environment: Rc::clone(&globals),
        }
    }

    pub fn execute<'b>(&mut self, stmt: &Stmt) -> EvaluationResult {
        match stmt {
            Stmt::Print(expr) => self.execute_print(expr),
            Stmt::Expression(expr) => self.evaluate(expr),
            Stmt::If(condition, then_branch, else_branch) => {
                self.execute_if(condition, then_branch, else_branch)
            }
            Stmt::While(condition, body) => self.execute_while(condition, body),
            Stmt::Var(identifier, initializer) => {
                let value = match initializer {
                    Some(initializer) => self.evaluate(initializer)?,
                    _ => Literal::Nil,
                };
                self.environment
                    .borrow_mut()
                    .define(&identifier.lexeme, value);
                Ok(Literal::Nil)
            }
            Stmt::Block(statements) => {
                let previous = self.environment.clone();
                let env = Environment::enclose(&self.environment);
                self.environment = Rc::new(RefCell::new(env));

                for stmt in statements {
                    match self.execute(&stmt) {
                        Ok(_) => (),
                        Err(reason) => {
                            self.environment = previous;
                            return Err(reason);
                        }
                    }
                }
                self.environment = previous;
                return Ok(Literal::Nil);
            }
        }
    }

    fn execute_print(&mut self, expr: &Expr) -> EvaluationResult {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(Literal::Nil)
    }

    fn execute_if(
        &mut self,
        condition: &Expr,
        then_branch: &Box<Stmt>,
        else_branch: &Option<Box<Stmt>>,
    ) -> EvaluationResult {
        let value = self.evaluate(condition)?;
        if value.is_truthy() {
            return self.execute(&*then_branch);
        }
        if let Some(else_branch) = else_branch {
            return self.execute(&*else_branch);
        }
        return Ok(Literal::Nil);
    }

    fn execute_while(&mut self, condition: &Expr, body: &Box<Stmt>) -> EvaluationResult {
        let body = &*body;
        while self.evaluate(condition)?.is_truthy() {
            self.execute(body)?;
        }
        Ok(Literal::Nil)
    }

    pub fn evaluate(&mut self, expr: &Expr) -> EvaluationResult {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Unary(operator, right) => self.evaluate_unary_expression(operator, right),
            Expr::Binary(left, operator, right) => {
                self.evaluate_binary_expression(left, operator, right)
            }
            Expr::Var(identifier) => self.evaluate_var(identifier),
            Expr::Assign(identifier, expr) => self.evaluate_assignment(identifier, expr),
            Expr::Logical(left, operator, right) => self.evaluate_logical(left, operator, right),
        }
    }

    fn evaluate_var(&mut self, identifier: &Token) -> EvaluationResult {
        match self.environment.borrow().fetch(&identifier.lexeme) {
            Some(value) => Ok(value.to_owned()),
            None => Err(LoxError::new(
                &identifier,
                LoxErrorType::RuntimeError,
                DetailedErrorType::UndeclaredIdentifier,
            )),
        }
    }

    fn evaluate_logical(
        &mut self,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> EvaluationResult {
        let value = self.evaluate(&*left)?;
        match operator.token_type {
            TokenType::Or => {
                if value.is_truthy() {
                    return Ok(value);
                }
            }
            _ => {
                if !value.is_truthy() {
                    return Ok(value);
                }
            }
        }
        return self.evaluate(&right);
    }

    fn evaluate_assignment(&mut self, identifier: &Token, expr: &Box<Expr>) -> EvaluationResult {
        let value = self.evaluate(&*expr)?;
        if self
            .environment
            .borrow_mut()
            .assign(&identifier.lexeme, value.clone())
        {
            Ok(value)
        } else {
            Err(LoxError::new(
                &identifier,
                LoxErrorType::RuntimeError,
                DetailedErrorType::UndeclaredIdentifier,
            ))
        }
    }

    fn evaluate_unary_expression(
        &mut self,
        operator: &Token,
        right: &Box<Expr>,
    ) -> EvaluationResult {
        let right = self.evaluate(&*right)?;
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
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> EvaluationResult {
        let left = self.evaluate(&*left)?;
        let right = self.evaluate(&*right)?;

        match operator.token_type {
            TokenType::Plus => match (&left, &right) {
                (Literal::String(left), Literal::String(right)) => {
                    let concatenated = format!("{}{}", left, right);
                    return Ok(Literal::String(concatenated));
                }
                _ => evaluate_arithmetic(operator, &left, &right),
            },
            TokenType::Minus | TokenType::Star | TokenType::Slash => {
                evaluate_arithmetic(operator, &left, &right)
            }
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => evaluate_comparison(operator, &left, &right),
            TokenType::EqualEqual => Ok(Literal::Boolean(left == right)),
            TokenType::BangEqual => Ok(Literal::Boolean(left != right)),
            _ => panic!(),
        }
    }
}
