//! Virtual machine to support running l9 toy programming language
use crate::log::LoggingTracer;
use crate::trace::VmStepTrace;
use crate::value::ValueType;
use crate::vm::opcode::{Chunk, Op};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum VmError {
    #[error("compilation failed")]
    CompilationError,
    #[error("runtime error")]
    RuntimeError(VmRuntimeError),
}

#[derive(Debug, Clone, PartialEq, Error, Default)]
pub enum VmRuntimeError {
    #[default]
    #[error("unknown error")]
    Unknown,
    #[error("stack exhausted")]
    StackExhausted,
    #[error("operation is not implemented for operand type")]
    TypeMismatch,
    #[error("variable {0} is not defined")]
    UndefinedVariable(String),
    #[error("wrong operation")]
    WrongOperation,
}

#[derive(Debug)]
pub struct Vm {
    ip: usize,
    chunk: Chunk,
    stack: VmStack,
    globals: HashMap<String, ValueType>,
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
                Op::LoadBool(b) => {
                    let value = ValueType::Bool(*b);
                    self.stack.push(value);
                }
                Op::Nil => {
                    self.stack.push(ValueType::Nil);
                }
                Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Ge | Op::Le | Op::Cmp => {
                    self.binary_operation(op.clone())?
                }
                Op::Not => self.not()?,
                Op::Print => self.print()?,
                Op::Global(name) => self.global_variable(name.clone())?,
                Op::LoadGlobal(name) => self.load_global_variable(name.clone())?,
            }
            if let Some(trace) = &self.trace {
                trace.trace_after(self.ip, &self.chunk, &self.stack);
            }
        }
        Ok(())
    }

    fn binary_operation(&mut self, operation: Op) -> Result<(), VmError> {
        let value_a = self.stack.pop()?;
        let value_b = self.stack.pop()?;

        let result = match (operation, value_a, value_b) {
            (Op::Add, ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a + b),
            (Op::Sub, ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a - b),
            (Op::Mul, ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a * b),
            (Op::Div, ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a / b),
            (Op::Ge, ValueType::Number(a), ValueType::Number(b)) => ValueType::Bool(a >= b),
            (Op::Le, ValueType::Number(a), ValueType::Number(b)) => ValueType::Bool(a <= b),
            (Op::Cmp, ValueType::Number(a), ValueType::Number(b)) => ValueType::Bool(a == b),
            (Op::Cmp, ValueType::Bool(a), ValueType::Bool(b)) => ValueType::Bool(a == b),
            (Op::Not, _, _) => {
                return Err(VmError::RuntimeError(VmRuntimeError::WrongOperation));
            }
            _ => {
                return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
            }
        };
        self.stack.push(result);
        Ok(())
    }

    fn not(&mut self) -> Result<(), VmError> {
        let result = match self.stack.pop()? {
            ValueType::Bool(b) => ValueType::Bool(!b),
            _ => {
                return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
            }
        };
        self.stack.push(result);
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

    fn global_variable(&mut self, name: String) -> Result<(), VmError> {
        let value = self.stack.pop()?;
        self.globals.insert(name, value);
        Ok(())
    }

    fn load_global_variable(&mut self, name: String) -> Result<(), VmError> {
        let value = self.globals.get(&name).ok_or(VmError::RuntimeError(
            VmRuntimeError::UndefinedVariable(name.clone()),
        ))?;
        self.stack.push(value.clone());
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
            globals: HashMap::new(),
            trace: Some(Box::new(tracer)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::opcode::Op;

    #[test]
    fn interpret_correct_program() {
        let program = Chunk::new(vec![Op::Return]);
        let mut vm = Vm::default();
        let result = vm.interpret(program);
        assert!(result.is_ok());
    }
}
