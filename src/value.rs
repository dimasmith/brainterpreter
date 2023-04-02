//! Different values natively supported by the virtual machine

use std::fmt::Display;

use crate::vm::opcode::Chunk;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Nil,
    Bool(bool),
    Number(f64),
    Address(usize),
    Text(Box<String>),
    Function(Box<Function>),
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    chunk: Chunk,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Nil => write!(f, "nil"),
            ValueType::Bool(b) => write!(f, "b:{}", b),
            ValueType::Number(n) => write!(f, "f:{}", n),
            ValueType::Address(a) => write!(f, "*:{}", a),
            ValueType::Text(s) => write!(f, "s:{}", s),
            ValueType::Function(func) => write!(f, "fn:{}", func.name),
        }
    }
}

impl Function {
    pub fn new(name: String, chunk: Chunk) -> Self {
        Self { name, chunk }
    }

    pub fn script(chunk: Chunk) -> Self {
        Self {
            name: "$script$".to_string(),
            chunk,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }
}

impl PartialEq<Function> for Function {
    fn eq(&self, other: &Function) -> bool {
        self.name == other.name
    }
}
