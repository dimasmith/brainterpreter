//! Different values natively supported by the virtual machine

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Nil,
    Bool(bool),
    Number(f64),
    Address(usize),
    Text(Box<String>),
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Nil => write!(f, "nil"),
            ValueType::Bool(b) => write!(f, "b:{}", b),
            ValueType::Number(n) => write!(f, "f:{}", n),
            ValueType::Address(a) => write!(f, "*:{}", a),
            ValueType::Text(s) => write!(f, "s:{}", s),
        }
    }
}
