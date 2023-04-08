//! Different values natively supported by the virtual machine

use std::fmt::Display;

use thiserror::Error;

use crate::vm::opcode::Chunk;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Nil,
    Bool(bool),
    Number(f64),
    Address(usize),
    Text(Box<String>),
    Function(Box<Function>),
    NativeFunction(Box<NativeFunction>),
}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("only number can be an index. {0} cannot be used as index")]
    InvalidIndexType(ValueType),
    #[error("index must be a positive number. {0} is not a valid index")]
    IncorrectIndex(f64),
    #[error("index `{index}` is out of bounds. index must be in range [0, {size})")]
    IndexOutOfBounds { index: usize, size: usize },
    #[error("only arrays and strings can be indexed. {0} cannot be indexed")]
    UnsupportedArrayType(ValueType),
    #[error("array does not support value of type `{0}`")]
    UnsupportedArrayValueType(ValueType),
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    chunk: Chunk,
    arity: usize,
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    name: String,
    arity: usize,
}

impl ValueType {
    fn index(&self) -> Result<usize, TypeError> {
        match self {
            ValueType::Number(num) => {
                let idx = *num as isize;
                if idx < 0 {
                    return Err(TypeError::IncorrectIndex(*num));
                }
                Ok(idx as usize)
            }
            _ => Err(TypeError::InvalidIndexType(self.clone())),
        }
    }

    pub fn get(&self, index: &ValueType) -> Result<ValueType, TypeError> {
        match self {
            ValueType::Text(s) => {
                let idx = self.index_in_bounds(index.index()?)?;
                Ok(ValueType::Text(Box::new(
                    s.chars().nth(idx).unwrap().to_string(),
                )))
            }
            _ => Err(TypeError::UnsupportedArrayType(self.clone())),
        }
    }

    pub fn set(&self, index: &ValueType, value: ValueType) -> Result<ValueType, TypeError> {
        match (self, value) {
            (ValueType::Text(s), ValueType::Text(v)) => {
                let idx = self.index_in_bounds(index.index()?)?;
                let mut s = s.clone();
                s.replace_range(idx..idx + 1, &v);
                Ok(ValueType::Text(s))
            }
            (ValueType::Text(_), v) => Err(TypeError::UnsupportedArrayValueType(v)),
            _ => Err(TypeError::UnsupportedArrayType(self.clone())),
        }
    }

    fn index_in_bounds(&self, index: usize) -> Result<usize, TypeError> {
        match self {
            ValueType::Text(s) => {
                if index >= s.len() {
                    return Err(TypeError::IndexOutOfBounds {
                        index,
                        size: s.len(),
                    });
                }
                Ok(index)
            }
            _ => Err(TypeError::UnsupportedArrayType(self.clone())),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            ValueType::Nil => "nil".to_string(),
            ValueType::Bool(b) => b.to_string(),
            ValueType::Number(n) => n.to_string(),
            ValueType::Address(a) => a.to_string(),
            ValueType::Text(s) => s.to_string(),
            ValueType::Function(func) => func.name.to_string(),
            ValueType::NativeFunction(func) => func.name.to_string(),
        }
    }
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
            ValueType::NativeFunction(func) => write!(f, "<native>fn:{}", func.name),
        }
    }
}

impl Function {
    pub fn new(name: String, chunk: Chunk, arity: usize) -> Self {
        Self { name, chunk, arity }
    }

    pub fn script(chunk: Chunk) -> Self {
        Self {
            name: "$main$".to_string(),
            chunk,
            arity: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }

    pub fn arity(&self) -> usize {
        self.arity
    }
}

impl NativeFunction {
    pub fn new(name: String, arity: usize) -> Self {
        Self { name, arity }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arity(&self) -> usize {
        self.arity
    }
}

impl PartialEq<Function> for Function {
    fn eq(&self, other: &Function) -> bool {
        self.name == other.name
    }
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let num = ValueType::Number(1.0);
        let idx = num.index();
        assert!(matches!(idx, Ok(1)));

        let num = ValueType::Number(-1.0);
        let idx = num.index();
        assert!(matches!(idx, Err(TypeError::IncorrectIndex(-1.0))));

        let num = ValueType::Text(Box::new("hello".to_string()));
        let idx = num.index();
        assert!(matches!(
            idx,
            Err(TypeError::InvalidIndexType(ValueType::Text(_)))
        ));
    }

    #[test]
    fn get_string_elements() {
        let s = ValueType::Text(Box::new("hello".to_string()));
        let idx = ValueType::Number(0.0);
        let val = s.get(&idx);
        assert_eq!(val.unwrap(), ValueType::Text(Box::new("h".to_string())));

        let idx = ValueType::Number(1.0);
        let val = s.get(&idx);
        assert_eq!(val.unwrap(), ValueType::Text(Box::new("e".to_string())));

        let idx = ValueType::Number(16.0);
        let val = s.get(&idx);
        assert!(matches!(
            val,
            Err(TypeError::IndexOutOfBounds { index: 16, size: 5 })
        ));
    }

    #[test]
    fn set_string_elements() {
        let s = ValueType::Text(Box::new("hello".to_string()));
        let idx = ValueType::Number(0.0);
        let val = ValueType::Text(Box::new("H".to_string()));
        let new_s = s.set(&idx, val);
        assert_eq!(
            new_s.unwrap(),
            ValueType::Text(Box::new("Hello".to_string()))
        );

        let idx = ValueType::Number(16.0);
        let val = ValueType::Text(Box::new("H".to_string()));
        let new_s = s.set(&idx, val);
        assert!(matches!(
            new_s,
            Err(TypeError::IndexOutOfBounds { index: 16, size: 5 })
        ));

        let idx = ValueType::Number(16.0);
        let val = ValueType::Number(10.0);
        let new_s = s.set(&idx, val);
        assert!(
            matches!(
                new_s,
                Err(TypeError::UnsupportedArrayValueType(ValueType::Number(_)))
            ),
            "string does not support types other than string"
        );
    }
}
