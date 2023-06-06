use std::cell::RefCell;

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

pub struct Interpreter;

impl Interpreter {
    pub fn evaluate_statement<'b>(env: &RefCell<Environment>, stmt: Stmt) -> EvaluationResult {
        match stmt {
            Stmt::Print(expr) => Self::evaluate_print(env, expr),
            Stmt::Expression(expr) => Self::evaluate(env, expr),
            Stmt::If(condition, then_branch, else_branch) => {
                Self::evaluate_if(env, condition, then_branch, else_branch)
            }
            Stmt::Var(identifier, Some(initializer)) => match Self::evaluate(env, initializer) {
                Ok(value) => {
                    env.borrow_mut().define(&identifier.lexeme, value);
                    Ok(Literal::Nil)
                }
                Err(reason) => Err(reason),
            },
            Stmt::Var(identifier, None) => {
                env.borrow_mut().define(&identifier.lexeme, Literal::Nil);
                Ok(Literal::Nil)
            }
            Stmt::Block(statements) => {
                env.borrow_mut().add_frame();

                for stmt in statements {
                    match Self::evaluate_statement(env, stmt) {
                        Ok(_) => (),
                        Err(reason) => {
                            env.borrow_mut().pop_frame();
                            return Err(reason);
                        }
                    }
                }
                env.borrow_mut().pop_frame();
                return Ok(Literal::Nil);
            }
        }
    }

    fn evaluate_print(env: &RefCell<Environment>, expr: Expr) -> EvaluationResult {
        let value = Self::evaluate(env, expr)?;
        println!("{}", value);
        Ok(Literal::Nil)
    }

    fn evaluate_if(
        env: &RefCell<Environment>,
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    ) -> EvaluationResult {
        let value = Self::evaluate(env, condition)?;
        if value.is_truthy() {
            return Self::evaluate_statement(env, *then_branch);
        }
        if let Some(else_branch) = else_branch {
            return Self::evaluate_statement(env, *else_branch);
        }
        return Ok(Literal::Nil);
    }

    pub fn evaluate(env: &RefCell<Environment>, expr: Expr) -> EvaluationResult {
        match expr {
            Expr::Literal(value) => Ok(value.to_owned()),
            Expr::Grouping(expr) => Self::evaluate(env, *expr),
            Expr::Unary(operator, right) => Self::evaluate_unary_expression(env, operator, right),
            Expr::Binary(left, operator, right) => {
                Self::evaluate_binary_expression(env, left, operator, right)
            }
            Expr::Var(identifier) => Self::evaluate_var(env, identifier),
            Expr::Assign(identifier, expr) => Self::evaluate_assignment(env, identifier, expr),
            Expr::Logical(left, operator, right) => {
                Self::evaluate_logical(env, left, operator, right)
            }
        }
    }

    fn evaluate_var(env: &RefCell<Environment>, identifier: Token) -> EvaluationResult {
        match env.borrow().fetch(&identifier.lexeme) {
            Some(value) => Ok(value.to_owned()),
            None => Err(LoxError::new(
                &identifier,
                LoxErrorType::RuntimeError,
                DetailedErrorType::UndeclaredIdentifier,
            )),
        }
    }

    fn evaluate_logical(
        env: &RefCell<Environment>,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    ) -> EvaluationResult {
        let value = Self::evaluate(env, *left)?;
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
        return Self::evaluate(env, *right);
    }

    fn evaluate_assignment(
        env: &RefCell<Environment>,
        identifier: Token,
        expr: Box<Expr>,
    ) -> EvaluationResult {
        let value = Self::evaluate(env, *expr)?;
        if env.borrow_mut().assign(&identifier.lexeme, value.clone()) {
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
        env: &RefCell<Environment>,
        operator: Token,
        right: Box<Expr>,
    ) -> EvaluationResult {
        let right = Self::evaluate(env, *right)?;
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
        env: &RefCell<Environment>,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    ) -> EvaluationResult {
        let left = Self::evaluate(env, *left)?;
        let right = Self::evaluate(env, *right)?;

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
