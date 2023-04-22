//! Different values natively supported by the virtual machine

use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use thiserror::Error;

use crate::vm::opcode::Chunk;
use crate::vm::{Vm, VmRuntimeError};

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Nil,
    Bool(bool),
    Number(f64),
    Address(usize),
    Text(Box<String>),
    Function(Box<Function>),
    NativeFunction(Rc<NativeFunction>),
    Array(Box<Vec<ValueType>>),
    ArrayRef(Rc<RefCell<Vec<ValueType>>>),
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

#[derive(Clone)]
pub struct NativeFunction {
    name: String,
    arity: usize,
    function: fn(&mut Vm) -> Result<(), VmRuntimeError>,
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
            ValueType::Array(arr) => {
                let idx = self.index_in_bounds(index.index()?)?;
                Ok(arr[idx].clone())
            }
            ValueType::ArrayRef(arr) => {
                let idx = self.index_in_bounds(index.index()?)?;
                Ok(arr.borrow()[idx].clone())
            }
            _ => Err(TypeError::UnsupportedArrayType(self.clone())),
        }
    }

    pub fn set(&self, index: &ValueType, value: ValueType) -> Result<ValueType, TypeError> {
        match (self, &value) {
            (ValueType::Text(s), ValueType::Text(v)) => {
                let idx = self.index_in_bounds(index.index()?)?;
                let mut s = s.clone();
                s.replace_range(idx..idx + 1, v);
                Ok(ValueType::Text(s))
            }
            (ValueType::Array(arr), v) => {
                let idx = self.index_in_bounds(index.index()?)?;
                let mut arr = arr.clone();
                arr[idx] = v.clone();
                Ok(ValueType::Array(arr))
            }
            (ValueType::ArrayRef(arr), v) => {
                let idx = self.index_in_bounds(index.index()?)?;
                arr.borrow_mut()[idx] = v.clone();
                Ok(self.clone())
            }
            (ValueType::Text(_), _) => Err(TypeError::UnsupportedArrayValueType(value)),
            _ => Err(TypeError::UnsupportedArrayType(self.clone())),
        }
    }

    fn index_in_bounds(&self, index: usize) -> Result<usize, TypeError> {
        match self {
            ValueType::Text(_) | ValueType::Array(_) | ValueType::ArrayRef(_) => {
                let len = self.len()?;
                if index >= len {
                    return Err(TypeError::IndexOutOfBounds { index, size: len });
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
            ValueType::Array(_) => "[]".to_string(),
            ValueType::ArrayRef(_) => "&[]".to_string(),
        }
    }

    fn len(&self) -> Result<usize, TypeError> {
        match self {
            ValueType::Text(s) => Ok(s.len()),
            ValueType::Array(arr) => Ok(arr.len()),
            ValueType::ArrayRef(arr) => Ok(arr.borrow().len()),
            _ => Err(TypeError::UnsupportedArrayType(self.clone())),
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
            ValueType::Array(_) => write!(f, "[]"),
            ValueType::ArrayRef(_) => write!(f, "&[]"),
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
    pub fn new(
        name: &str,
        arity: usize,
        function: fn(&mut Vm) -> Result<(), VmRuntimeError>,
    ) -> Self {
        Self {
            name: name.to_string(),
            arity,
            function,
        }
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

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native>fn:{}", self.name)
    }
}

impl NativeFunction {
    pub fn call(&self, vm: &mut Vm) -> Result<(), VmRuntimeError> {
        (self.function)(vm)
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
        assert!(matches!(idx, Err(TypeError::IncorrectIndex(_))));

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

    #[test]
    fn values_as_string() {
        let s = ValueType::Text(Box::new("hello".to_string()));
        assert_eq!(s.as_string(), "hello");

        let s = ValueType::Number(10.0);
        assert_eq!(s.as_string(), "10");

        let s = ValueType::Bool(true);
        assert_eq!(s.as_string(), "true");

        let s = ValueType::Nil;
        assert_eq!(s.as_string(), "nil");

        let s = ValueType::Address(10);
        assert_eq!(s.as_string(), "10");

        let s = ValueType::Function(Box::new(Function::new(
            "test".to_string(),
            Chunk::default(),
            0,
        )));
        assert_eq!(s.as_string(), "test");

        let s = ValueType::NativeFunction(Rc::new(NativeFunction::new("test", 0, |_vm| Ok(()))));
        assert_eq!(s.as_string(), "test");

        let s = ValueType::Array(Box::new(vec![ValueType::Number(10.0)]));
        assert_eq!(s.as_string(), "[]");

        let s = ValueType::ArrayRef(Rc::new(RefCell::new(vec![ValueType::Number(10.0)])));
        assert_eq!(s.as_string(), "&[]");
    }

    #[test]
    fn display() {
        let s = ValueType::Text(Box::new("hello".to_string()));
        assert_eq!(format!("{}", s), "s:hello");

        let s = ValueType::Number(10.0);
        assert_eq!(format!("{}", s), "f:10");

        let s = ValueType::Bool(true);
        assert_eq!(format!("{}", s), "b:true");

        let s = ValueType::Nil;
        assert_eq!(format!("{}", s), "nil");

        let s = ValueType::Address(10);
        assert_eq!(format!("{}", s), "*:10");

        let s = ValueType::Function(Box::new(Function::new(
            "test".to_string(),
            Chunk::default(),
            0,
        )));
        assert_eq!(format!("{}", s), "fn:test");

        let s = ValueType::NativeFunction(Rc::new(NativeFunction::new("test", 0, |_vm| Ok(()))));
        assert_eq!(format!("{}", s), "<native>fn:test");

        let s = ValueType::Array(Box::new(vec![ValueType::Number(10.0)]));
        assert_eq!(format!("{}", s), "[]");

        let s = ValueType::ArrayRef(Rc::new(RefCell::new(vec![ValueType::Number(10.0)])));
        assert_eq!(format!("{}", s), "&[]");
    }
}
