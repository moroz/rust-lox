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
