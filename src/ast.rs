//! Abstract syntax tree for l9 language

use crate::ast::AstExpression::{BinaryOperation, NumberLiteral, UnaryOperation};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstExpression {
    NumberLiteral(f64),
    BinaryOperation(Operation, Box<AstExpression>, Box<AstExpression>),
    UnaryOperation(Operation, Box<AstExpression>),
    Cmp(Box<AstExpression>, Box<AstExpression>),
}

impl AstExpression {
    pub fn binary(op: Operation, lhs: AstExpression, rhs: AstExpression) -> Self {
        BinaryOperation(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn unary(op: Operation, lhs: AstExpression) -> Self {
        UnaryOperation(op, Box::new(lhs))
    }

    pub fn number(n: impl Into<f64>) -> Self {
        NumberLiteral(n.into())
    }
}
