use crate::{
    errors::DetailedErrorType,
    errors::LoxError,
    errors::LoxErrorType,
    expr::Expr,
    literal::Literal,
    token::{Token, TokenType},
};

pub type EvaluationResult = Result<Literal, LoxError>;

fn evaluate_arithmetic(operator: &Token, left: Literal, right: Literal) -> EvaluationResult {
    match (left, right) {
        (Literal::Number(left), Literal::Number(right)) => match operator.token_type {
            TokenType::Plus => Ok(Literal::Number(left + right)),
            TokenType::Minus => Ok(Literal::Number(left - right)),
            TokenType::Slash => Ok(Literal::Number(left / right)),
            TokenType::Star => Ok(Literal::Number(left * right)),
            _ => panic!(),
        },

        _ => Err(LoxError::new(
            operator,
            LoxErrorType::RuntimeError,
            DetailedErrorType::ExpectedNumber,
        )),
    }
}

fn evaluate_comparison(operator: &Token, left: Literal, right: Literal) -> EvaluationResult {
    match (left, right) {
        (Literal::Number(left), Literal::Number(right)) => match operator.token_type {
            TokenType::Less => Ok(Literal::Boolean(left < right)),
            TokenType::LessEqual => Ok(Literal::Boolean(left <= right)),
            TokenType::Greater => Ok(Literal::Boolean(left > right)),
            TokenType::GreaterEqual => Ok(Literal::Boolean(left >= right)),
            _ => panic!(),
        },

        _ => Err(LoxError::new(
            operator,
            LoxErrorType::RuntimeError,
            DetailedErrorType::ExpectedNumber,
        )),
    }
}

impl Expr {
    pub fn evaluate(&self) -> EvaluationResult {
        match self {
            Expr::Literal(value) => Ok(value.to_owned()),
            Expr::Grouping(expr) => expr.evaluate(),
            Expr::Unary(operator, right) => self.evaluate_unary_expression(operator, right),
            Expr::Binary(left, operator, right) => {
                self.evaluate_binary_expression(left, operator, right)
            }
        }
    }

    fn evaluate_unary_expression(&self, operator: &Token, right: &Box<Expr>) -> EvaluationResult {
        let right = right.evaluate();
        if right.is_err() {
            return right;
        }

        let right = right.unwrap();
        match operator.token_type {
            TokenType::Minus => match right {
                Literal::Number(value) => Ok(Literal::Number(-value)),
                _ => Err(LoxError::new(
                    operator,
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
        &self,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> EvaluationResult {
        let left = left.evaluate();
        if left.is_err() {
            return left;
        }
        let right = right.evaluate();
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
