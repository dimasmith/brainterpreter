//! Virtual machine to support running l9 toy programming language
use crate::chunk::Chunk;
use crate::ops::Op;
use crate::value::ValueType;
use log::debug;
use thiserror::Error;

#[derive(Debug, Copy, Clone, PartialEq, Error)]
pub enum VmError {
    #[error("compilation failed")]
    CompilationError,
    #[error("runtime error")]
    RuntimeError(VmRuntimeError),
}

#[derive(Debug, Copy, Clone, PartialEq, Error, Default)]
pub enum VmRuntimeError {
    #[default]
    #[error("unknown error")]
    Unknown,
    #[error("stack exhausted")]
    StackExhausted,
    #[error("operation is not implemented for operand type")]
    TypeMismatch,
}

#[derive(Debug, Default)]
pub struct Vm {
    ip: usize,
    chunk: Chunk,
    stack: VmStack,
}

const STACK_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
struct VmStack {
    stack: Vec<ValueType>,
}

impl VmStack {
    fn pop(&mut self) -> Result<ValueType, VmError> {
        self.stack
            .pop()
            .ok_or(VmError::RuntimeError(VmRuntimeError::StackExhausted))
    }

    fn push(&mut self, value: ValueType) {
        self.stack.push(value);
    }
}

impl Default for VmStack {
    fn default() -> Self {
        let stack = Vec::with_capacity(STACK_SIZE);
        VmStack { stack }
    }
}

impl Vm {
    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), VmError> {
        self.ip = 0;
        self.chunk = chunk;
        debug!("Disasembling program \n{}", &self.chunk);
        self.run()?;
        Ok(())
    }

    fn run(&mut self) -> Result<(), VmError> {
        debug!("Tracing execution:");
        while let Some(op) = self.chunk.op(self.ip) {
            self.ip += 1;
            debug!(":{}", op);
            match op {
                Op::Return => {
                    dbg!(&self.stack);
                    break;
                }
                Op::LoadFloat(n) => {
                    let value = ValueType::Number(*n);
                    self.stack.push(value);
                }
                Op::Add => self.add()?,
                Op::Cmp => self.compare()?,
                Op::Neg => self.negate()?,
            }
        }
        Ok(())
    }

    fn add(&mut self) -> Result<(), VmError> {
        let value_a = self.stack.pop()?;
        let value_b = self.stack.pop()?;

        match (value_a, value_b) {
            (ValueType::Number(a), ValueType::Number(b)) => {
                let result = a + b;
                let result_value = ValueType::Number(result);
                self.stack.push(result_value);
            }
            _ => {
                return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
            }
        }
        Ok(())
    }

    fn compare(&mut self) -> Result<(), VmError> {
        let value_a = self.stack.pop()?;
        let value_b = self.stack.pop()?;

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
            _ => {
                return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
            }
        }
        Ok(())
    }

    fn negate(&mut self) -> Result<(), VmError> {
        match self.stack.pop()? {
            ValueType::Number(n) => {
                let value = -n;
                self.stack.push(ValueType::Number(value));
            }
            _ => {
                return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
            }
        }
        Ok(())
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
