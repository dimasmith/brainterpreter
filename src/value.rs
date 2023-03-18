//! Different values natively supported by the virtual machine

use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ValueType {
    Bool(bool),
    Number(f64),
    Address(usize),
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Bool(b) => write!(f, "b:{}", b),
            ValueType::Number(n) => write!(f, "f:{}", n),
            ValueType::Address(a) => write!(f, "*:{}", a),
        }
    }
}
