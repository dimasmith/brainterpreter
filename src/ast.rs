//! Abstract syntax tree for l9 language

use crate::ast::Expression::{BinaryOperation, NumberLiteral, UnaryOperation};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    NumberLiteral(f64),
    BinaryOperation(Operation, Box<Expression>, Box<Expression>),
    UnaryOperation(Operation, Box<Expression>),
    Cmp(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn binary(op: Operation, lhs: Expression, rhs: Expression) -> Self {
        BinaryOperation(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn unary(op: Operation, lhs: Expression) -> Self {
        UnaryOperation(op, Box::new(lhs))
    }

    pub fn number(n: impl Into<f64>) -> Self {
        NumberLiteral(n.into())
    }
}
