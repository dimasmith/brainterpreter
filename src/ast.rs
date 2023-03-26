//! Abstract syntax tree for l9 language

#[derive(Debug, Clone, PartialEq)]
pub enum AstExpression {
    NumberLiteral(f64),
    Add(Box<AstExpression>, Box<AstExpression>),
    Cmp(Box<AstExpression>, Box<AstExpression>),
}
