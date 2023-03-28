//! Virtual machine to support running l9 toy programming language
use crate::chunk::Chunk;
use crate::log::LoggingTracer;
use crate::ops::Op;
use crate::trace::VmStepTrace;
use crate::value::ValueType;
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

#[derive(Debug)]
pub struct Vm {
    ip: usize,
    chunk: Chunk,
    stack: VmStack,
    trace: Option<Box<dyn VmStepTrace>>,
}

const STACK_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
pub struct VmStack {
    stack: Vec<ValueType>,
}

impl VmStack {
    pub fn pop(&mut self) -> Result<ValueType, VmError> {
        self.stack
            .pop()
            .ok_or(VmError::RuntimeError(VmRuntimeError::StackExhausted))
    }

    pub fn peek(&self, offset: usize) -> Option<&ValueType> {
        self.stack.get(offset)
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push(&mut self, value: ValueType) {
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
        self.run()?;
        Ok(())
    }

    fn run(&mut self) -> Result<(), VmError> {
        while let Some(op) = self.chunk.op(self.ip) {
            if let Some(trace) = &self.trace {
                trace.trace_before(self.ip, &self.chunk, &self.stack);
            }
            self.ip += 1;
            match op {
                Op::Return => {
                    if let Some(v) = &self.stack.peek(0) {
                        println!("{}", v);
                    } else {
                        println!("None");
                    }
                }
                Op::LoadFloat(n) => {
                    let value = ValueType::Number(*n);
                    self.stack.push(value);
                }
                Op::Add => self.add()?,
                Op::Sub => self.sub()?,
                Op::Mul => self.mul()?,
                Op::Div => self.div()?,
                Op::Cmp => self.compare()?,
                Op::Neg => self.negate()?,
                Op::Print => self.print()?,
            }
            if let Some(trace) = &self.trace {
                trace.trace_after(self.ip, &self.chunk, &self.stack);
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

    fn sub(&mut self) -> Result<(), VmError> {
        let value_a = self.stack.pop()?;
        let value_b = self.stack.pop()?;

        match (value_a, value_b) {
            (ValueType::Number(a), ValueType::Number(b)) => {
                let result = a - b;
                let result_value = ValueType::Number(result);
                self.stack.push(result_value);
            }
            _ => {
                return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
            }
        }
        Ok(())
    }

    fn mul(&mut self) -> Result<(), VmError> {
        let value_a = self.stack.pop()?;
        let value_b = self.stack.pop()?;

        match (value_a, value_b) {
            (ValueType::Number(a), ValueType::Number(b)) => {
                let result = a * b;
                let result_value = ValueType::Number(result);
                self.stack.push(result_value);
            }
            _ => {
                return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
            }
        }
        Ok(())
    }

    fn div(&mut self) -> Result<(), VmError> {
        let value_a = self.stack.pop()?;
        let value_b = self.stack.pop()?;

        match (value_a, value_b) {
            (ValueType::Number(a), ValueType::Number(b)) => {
                let result = a / b;
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

    fn print(&mut self) -> Result<(), VmError> {
        match self.stack.pop()? {
            ValueType::Number(n) => {
                println!("{}", n);
            }
            ValueType::Bool(b) => {
                println!("{}", b);
            }
            ValueType::Address(a) => {
                println!("{}", a);
            }
            _ => {
                return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
            }
        }
        Ok(())
    }
}

impl Default for Vm {
    fn default() -> Self {
        let tracer = LoggingTracer::default();
        Vm {
            ip: 0,
            chunk: Chunk::default(),
            stack: VmStack::default(),
            trace: Some(Box::new(tracer)),
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
