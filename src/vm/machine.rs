//! Virtual machine to support running l9 toy programming language
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::rc::Rc;

use thiserror::Error;

use crate::log::LoggingTracer;
use crate::trace::VmStepTrace;
use crate::value::{Function, ValueType};
use crate::vm::opcode::{Chunk, Op};

#[derive(Debug, Error)]
pub enum VmError {
    #[error("compilation failed")]
    CompilationError,
    #[error("runtime error")]
    RuntimeError(VmRuntimeError),
}

#[derive(Debug, Error, Default)]
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
    #[error("illegal jump from address {0} with offset {1}")]
    IllegalJump(usize, isize),
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("undefined constant at index {0}")]
    UndefinedConstant(usize),
}

pub struct Vm {
    stack: VmStack,
    globals: HashMap<String, ValueType>,
    frames: Vec<CallFrame>,
    trace: Option<Box<dyn VmStepTrace>>,
    out: Rc<RefCell<dyn Write>>,
}

const STACK_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
pub struct VmStack {
    stack: Vec<ValueType>,
}

#[derive(Debug)]
struct CallFrame {
    ip: usize,
    chunk: Chunk,
}

impl Vm {
    pub fn run_script(&mut self, script: Function) -> Result<(), VmError> {
        let call_frame = CallFrame::new(script.chunk().clone());
        self.frames.push(call_frame);
        self.run()?;
        Ok(())
    }

    fn run(&mut self) -> Result<(), VmError> {
        while let Some(op) = self.advance() {
            let op = op.clone();
            self.trace_before();
            match op {
                Op::Return => self.ret()?,
                Op::Call => self.call_function()?,
                Op::Const(n) => {
                    let value = self.constant(n)?;
                    self.stack.push(value);
                }
                Op::ConstFloat(n) => {
                    let value = ValueType::Number(n);
                    self.stack.push(value);
                }
                Op::ConstBool(b) => {
                    let value = ValueType::Bool(b);
                    self.stack.push(value);
                }
                Op::Pop => {
                    self.stack.pop()?;
                }
                Op::Nil => {
                    self.stack.push(ValueType::Nil);
                }
                Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Ge | Op::Le | Op::Cmp => {
                    self.binary_operation(op.clone())?
                }
                Op::Not => self.not()?,
                Op::Print => self.print()?,
                Op::StoreGlobal(name) => self.store_global(name.clone())?,
                Op::LoadGlobal(name) => self.load_global(name.clone())?,
                Op::StoreLocal(offset) => self.write_local_variable(offset)?,
                Op::LoadLocal(offset) => self.read_local_variable(offset)?,
                Op::Jump(offset) => self.jump(offset)?,
                Op::JumpIfFalse(offset) => self.jump_if_false(offset)?,
            }
            self.trace_after()
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
                self.out
                    .borrow_mut()
                    .write_fmt(format_args!("{}\n", n))
                    .map_err(|e| VmError::RuntimeError(VmRuntimeError::IoError(e)))?;
            }
            ValueType::Bool(b) => {
                self.out
                    .borrow_mut()
                    .write_fmt(format_args!("{}\n", b))
                    .map_err(|e| VmError::RuntimeError(VmRuntimeError::IoError(e)))?;
            }
            ValueType::Address(a) => {
                self.out
                    .borrow_mut()
                    .write_fmt(format_args!("{}\n", a))
                    .map_err(|e| VmError::RuntimeError(VmRuntimeError::IoError(e)))?;
            }
            ValueType::Nil => {
                self.out
                    .borrow_mut()
                    .write_fmt(format_args!("{}\n", "nil"))
                    .map_err(|e| VmError::RuntimeError(VmRuntimeError::IoError(e)))?;
            }
            ValueType::Function(f) => {
                self.out
                    .borrow_mut()
                    .write_fmt(format_args!("{}:{}\n", "fun", f.name()))
                    .map_err(|e| VmError::RuntimeError(VmRuntimeError::IoError(e)))?;
            }
            _ => {
                return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
            }
        }
        Ok(())
    }

    fn store_global(&mut self, name: String) -> Result<(), VmError> {
        let value = self.stack.pop()?;
        self.globals.insert(name, value);
        Ok(())
    }

    fn load_global(&mut self, name: String) -> Result<(), VmError> {
        let value = self.globals.get(&name).ok_or(VmError::RuntimeError(
            VmRuntimeError::UndefinedVariable(name.clone()),
        ))?;
        self.stack.push(value.clone());
        Ok(())
    }

    fn write_local_variable(&mut self, offset: usize) -> Result<(), VmError> {
        let value = self
            .stack
            .last()
            .ok_or(VmError::RuntimeError(VmRuntimeError::StackExhausted))?;
        self.stack.set(offset, value.clone())?;
        Ok(())
    }

    fn read_local_variable(&mut self, offset: usize) -> Result<(), VmError> {
        let value = self.stack.stack.get(offset).ok_or(VmError::RuntimeError(
            VmRuntimeError::UndefinedVariable(offset.to_string()),
        ))?;
        self.stack.push(value.clone());
        Ok(())
    }

    fn jump(&mut self, offset: i32) -> Result<(), VmError> {
        self.offset_ip(offset as isize)?;
        Ok(())
    }

    fn jump_if_false(&mut self, offset: i32) -> Result<(), VmError> {
        let value = self.stack.pop()?;
        if let ValueType::Bool(b) = value {
            if !b {
                self.offset_ip(offset as isize)?;
            }
        } else {
            return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
        }
        Ok(())
    }

    fn call_function(&mut self) -> Result<(), VmError> {
        let value = self.stack.pop()?;
        let function = match value {
            ValueType::Function(f) => f,
            _ => {
                return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
            }
        };
        let frame = CallFrame::new(function.chunk().clone());
        self.frames.push(frame);
        Ok(())
    }

    fn ret(&mut self) -> Result<(), VmError> {
        self.frames.pop();
        Ok(())
    }

    fn offset_ip(&mut self, offset: isize) -> Result<(), VmError> {
        let frame = self.frames.last_mut().unwrap();
        let ip = frame.ip;
        let new_ip = ip.checked_add_signed(offset).ok_or(VmError::RuntimeError(
            VmRuntimeError::IllegalJump(ip, offset),
        ))?;
        frame.ip = new_ip;
        Ok(())
    }

    fn advance(&mut self) -> Option<&Op> {
        self.frames.last_mut().and_then(|frame| frame.advance())
    }

    fn ip(&self) -> usize {
        self.frames.last().map(|frame| frame.ip).unwrap_or(0)
    }

    fn chunk(&self) -> Chunk {
        let frame = self.frames.last().unwrap();
        frame.chunk.clone()
    }

    fn trace_before(&self) {
        if let Some(ref tracer) = self.trace {
            tracer.trace_before(self.ip() - 1, &self.chunk(), &self.stack);
        }
    }

    fn trace_after(&mut self) {
        if let Some(trace) = &self.trace {
            trace.trace_after(self.ip(), &self.chunk(), &self.stack);
        }
    }

    fn constant(&self, index: usize) -> Result<ValueType, VmError> {
        let chunk = self.chunk();
        chunk.constant(index).cloned().ok_or(VmError::RuntimeError(
            VmRuntimeError::UndefinedConstant(index),
        ))
    }
}

impl Default for Vm {
    fn default() -> Self {
        let tracer = LoggingTracer::default();
        let out = stdout();
        Vm {
            stack: VmStack::default(),
            frames: Vec::new(),
            globals: HashMap::new(),
            trace: Some(Box::new(tracer)),
            out: Rc::new(RefCell::new(out)),
        }
    }
}

impl Vm {
    pub fn with_io<T>(out: Rc<RefCell<T>>) -> Self
    where
        T: Write + Send + Sync + 'static,
    {
        Vm {
            out,
            ..Default::default()
        }
    }
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

    fn last(&self) -> Option<&ValueType> {
        self.stack.last()
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

    fn set(&mut self, offset: usize, value: ValueType) -> Result<(), VmError> {
        if let Some(v) = self.stack.get_mut(offset) {
            *v = value;
            Ok(())
        } else {
            Err(VmError::RuntimeError(VmRuntimeError::StackExhausted))
        }
    }
}

impl CallFrame {
    fn new(chunk: Chunk) -> Self {
        CallFrame { chunk, ip: 0 }
    }

    #[allow(dead_code)]
    fn op(&self) -> Option<&Op> {
        self.chunk.op(self.ip)
    }

    fn advance(&mut self) -> Option<&Op> {
        let op = self.chunk.op(self.ip);
        self.ip += 1;
        op
    }
}

impl Default for VmStack {
    fn default() -> Self {
        let stack = Vec::with_capacity(STACK_SIZE);
        VmStack { stack }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod stack {
        use super::*;

        #[test]
        fn set_value_by_offset() {
            let mut stack = VmStack::default();
            stack.push(ValueType::Number(1.0));
            stack.push(ValueType::Number(2.0));
            stack.set(0, ValueType::Number(3.0)).unwrap();
            stack.set(1, ValueType::Number(4.0)).unwrap();
            assert_eq!(stack.stack[0], ValueType::Number(3.0));
            assert_eq!(stack.stack[1], ValueType::Number(4.0));
        }
    }
}
