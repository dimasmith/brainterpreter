//! Abstract syntax tree for l9 language

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstExpression {
    NumberLiteral(f64),
    BinaryOperation(Operation, Box<AstExpression>, Box<AstExpression>),
    Cmp(Box<AstExpression>, Box<AstExpression>),
}
