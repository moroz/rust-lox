use crate::{
    errors::DetailedErrorType,
    errors::LoxError,
    errors::LoxErrorType,
    expr::Expression,
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

impl Expression {
    pub fn evaluate(&self) -> EvaluationResult {
        match self {
            Expression::Literal(value) => Ok(value.to_owned()),
            Expression::Grouping(expr) => expr.evaluate(),
            Expression::Unary(operator, expr) => self.evaluate_unary_expression(),
            Expression::Binary(operator, left, right) => self.evaluate_binary_expression(),
        }
    }

    fn evaluate_unary_expression(&self) -> EvaluationResult {
        if let Expression::Unary(operator, right) = self {
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
        } else {
            panic!()
        }
    }

    fn evaluate_binary_expression(&self) -> EvaluationResult {
        if let Expression::Binary(left, operator, right) = self {
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
        } else {
            panic!()
        }
    }
}
