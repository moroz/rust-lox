use std::fmt::Debug;

use crate::{expr::Expr, token::Token};

#[derive(Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Print(expr) => {
                write!(f, "(print {:?})", expr)
            }
            Self::Expression(expr) => {
                write!(f, "({:?})", expr)
            }
            Self::Var(token, expr) => match expr {
                Some(expr) => {
                    write!(f, "(defvar {} {:?})", token.lexeme, expr)
                }
                None => {
                    write!(f, "(defvar {} nil)", token.lexeme)
                }
            },
            Self::Block(block) => {
                let body: Vec<_> = block.iter().map(|stmt| format!("{:?}", stmt)).collect();
                let joined = body.join(" ");
                write!(f, "(block {})", joined)
            }
            Self::If(condition, then_branch, else_branch) => match else_branch {
                Some(else_branch) => {
                    write!(
                        f,
                        "(if {:?} {:?} {:?})",
                        condition, then_branch, else_branch
                    )
                }
                None => {
                    write!(f, "(if {:?} {:?})", condition, then_branch)
                }
            },
            Self::While(condition, body) => {
                write!(f, "(while {:?} {:?})", condition, body)
            }
        }
    }
}
