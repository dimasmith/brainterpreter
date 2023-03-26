//! Abstract syntax tree for l9 language

use crate::ast::AstExpression::{BinaryOperation, NumberLiteral};

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
    Cmp(Box<AstExpression>, Box<AstExpression>),
}

impl AstExpression {
    pub fn binary_op(op: Operation, lhs: AstExpression, rhs: AstExpression) -> Self {
        BinaryOperation(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn number(n: impl Into<f64>) -> Self {
        NumberLiteral(n.into())
    }
}
