//! Virtual machine to support running l9 toy programming language
pub mod ops;
pub mod value;

use crate::ops::Op;
use crate::value::ValueType;
use thiserror::Error;

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    ops: Vec<Op>,
}

#[derive(Debug, Copy, Clone, PartialEq, Error)]
pub enum VmError {
    #[error("compilation failed")]
    CompilationError,
    #[error("runtime error")]
    RuntimeError,
}

impl Chunk {
    pub fn new(ops: Vec<Op>) -> Self {
        Chunk { ops }
    }

    pub fn op(&self, idx: usize) -> Option<&Op> {
        self.ops.get(idx)
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }
}

#[derive(Debug)]
pub struct Vm {
    ip: usize,
    chunk: Chunk,
    stack: VmStack,
}

const STACK_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
struct VmStack {
    stack: Vec<ValueType>,
    top: usize,
}

impl VmStack {
    fn pop(&mut self) -> ValueType {
        self.stack.pop().expect("missing stack value")
    }

    fn push(&mut self, value: ValueType) {
        self.stack.push(value);
    }
}

impl Default for VmStack {
    fn default() -> Self {
        let stack = Vec::with_capacity(STACK_SIZE);
        VmStack { stack, top: 0 }
    }
}

impl Vm {
    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), VmError> {
        self.ip = 0;
        self.chunk = chunk;
        dbg!(&self.chunk);
        self.run();
        Ok(())
    }

    fn run(&mut self) {
        while let Some(op) = self.chunk.op(self.ip) {
            self.ip += 1;
            println!(":{}", op);
            match op {
                Op::Return => {
                    dbg!(&self.stack);
                    break;
                }
                Op::LoadFloat(n) => {
                    let value = ValueType::Number(*n);
                    self.stack.push(value);
                }
                Op::Add => self.add(),
                Op::Cmp => self.compare(),
            }
        }
    }

    fn add(&mut self) {
        let value_a = self.stack.pop();
        let value_b = self.stack.pop();

        match (value_a, value_b) {
            (ValueType::Number(a), ValueType::Number(b)) => {
                let result = a + b;
                let result_value = ValueType::Number(result);
                self.stack.push(result_value);
            }
            _ => panic!("unsupported value types"),
        }
    }

    fn compare(&mut self) {
        let value_a = self.stack.pop();
        let value_b = self.stack.pop();

        match (value_a, value_b) {
            (ValueType::Number(a), ValueType::Number(b)) => {
                let result = a == b;
                let result_value = ValueType::Bool(result);
                self.stack.push(result_value);
            }
            (ValueType::Bool(a), ValueType::Bool(b)) => {
                let result = a == b;
                let result_value = ValueType::Bool(result);
                self.stack.push(result_value);
            }
            (ValueType::Address(a), ValueType::Address(b)) => {
                let result = a == b;
                let result_value = ValueType::Bool(result);
                self.stack.push(result_value);
            }
            _ => panic!("unsupported value types"),
        }
    }
}

impl Default for Vm {
    fn default() -> Self {
        Vm {
            ip: 0,
            chunk: Chunk::default(),
            stack: VmStack::default(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn interpret_correct_program() {
        let program = Chunk::new(vec![Op::Return]);
        let mut vm = Vm::default();
        let result = vm.interpret(program);
        assert!(result.is_ok());
    }
}
