//! Virtual machine to support running l9 toy programming language
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::rc::Rc;

use thiserror::Error;

use crate::log::LoggingTracer;
use crate::trace::VmStepTrace;
use crate::value::{Function, NativeFunction, TypeError, ValueType};
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
    #[error("accessing out of bounds value on index {0} with size {1}")]
    OutOfBounds(usize, f64),
    #[error("error accessing array {0}")]
    ArrayAccessError(#[from] TypeError),
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
    stack_top: usize,
}

impl Vm {
    pub fn run_script(&mut self, script: Function) -> Result<(), VmError> {
        let call_frame = CallFrame::new(script.chunk().clone(), 0);
        self.frames.push(call_frame);
        self.stack.push(ValueType::Function(Box::new(script)));
        self.run()?;
        Ok(())
    }

    fn run(&mut self) -> Result<(), VmError> {
        while let Some(op) = self.advance() {
            let op = op.clone();
            self.trace_before();
            match op {
                Op::Return => self.ret()?,
                Op::Call(arity) => self.call(arity)?,
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
                Op::LoadIndex => self.binary_operation(op.clone())?,
                Op::StoreIndex => self.store_index()?,
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

        let result = match (operation, &value_a, &value_b) {
            (Op::Add, ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a + b),
            (Op::Add, ValueType::Text(a), ValueType::Text(b)) => {
                let concat = format!("{}{}", a, b);
                ValueType::Text(Box::new(concat))
            }
            (Op::Sub, ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a - b),
            (Op::Mul, ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a * b),
            (Op::Div, ValueType::Number(a), ValueType::Number(b)) => ValueType::Number(a / b),
            (Op::Ge, ValueType::Number(a), ValueType::Number(b)) => ValueType::Bool(a >= b),
            (Op::Le, ValueType::Number(a), ValueType::Number(b)) => ValueType::Bool(a <= b),
            (Op::Cmp, ValueType::Number(a), ValueType::Number(b)) => ValueType::Bool(a == b),
            (Op::Cmp, ValueType::Bool(a), ValueType::Bool(b)) => ValueType::Bool(a == b),
            (Op::Cmp, ValueType::Text(a), ValueType::Text(b)) => ValueType::Bool(a == b),
            (Op::LoadIndex, ValueType::Text(s), ValueType::Number(i)) => value_a
                .get(&value_b)
                .map_err(|e| VmError::RuntimeError(VmRuntimeError::ArrayAccessError(e)))?,
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

    fn store_index(&mut self) -> Result<(), VmError> {
        let value = self.stack.pop()?;
        let target = self.stack.pop()?;
        let idx = self.stack.pop()?;
        let new_value = target
            .set(&idx, value)
            .map_err(|e| VmError::RuntimeError(VmRuntimeError::ArrayAccessError(e)))?;
        self.stack.push(new_value);
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
            ValueType::NativeFunction(f) => {
                self.out
                    .borrow_mut()
                    .write_fmt(format_args!("<native>{}:{}\n", "fun", f.name()))
                    .map_err(|e| VmError::RuntimeError(VmRuntimeError::IoError(e)))?;
            }
            ValueType::Text(s) => {
                self.out
                    .borrow_mut()
                    .write_fmt(format_args!("{}\n", s))
                    .map_err(|e| VmError::RuntimeError(VmRuntimeError::IoError(e)))?;
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
        let frame_offset = self.frames.last().unwrap().stack_top + offset + 1;
        self.stack.set(frame_offset, value.clone())?;
        Ok(())
    }

    fn read_local_variable(&mut self, offset: usize) -> Result<(), VmError> {
        let frame_offset = self.frames.last().unwrap().stack_top + offset + 1;
        let value = self
            .stack
            .stack
            .get(frame_offset)
            .ok_or(VmError::RuntimeError(VmRuntimeError::UndefinedVariable(
                frame_offset.to_string(),
            )))?;
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

    fn call(&mut self, arity: usize) -> Result<(), VmError> {
        let value = self.peek_value(arity)?.clone();
        match &value {
            ValueType::Function(f) => self.call_function(f, arity),
            ValueType::NativeFunction(f) => self.call_native_function(f, arity),
            _ => Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch)),
        }
    }

    fn peek_value(&mut self, arity: usize) -> Result<&ValueType, VmError> {
        self.stack
            .peek(arity)
            .ok_or(VmError::RuntimeError(VmRuntimeError::StackExhausted))
    }

    fn call_function(&mut self, function: &Function, arity: usize) -> Result<(), VmError> {
        if arity != function.arity() {
            return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
        }
        let stack_top = self.stack.len() - function.arity() - 1;
        let frame = CallFrame::new(function.chunk().clone(), stack_top);
        self.frames.push(frame);
        Ok(())
    }

    fn call_native_function(
        &mut self,
        function: &NativeFunction,
        arity: usize,
    ) -> Result<(), VmError> {
        if arity != function.arity() {
            return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch));
        }
        match function.name() {
            "len" => {
                let value = self.stack.pop()?;
                match value {
                    ValueType::Text(s) => {
                        self.stack.pop()?;
                        self.stack.push(ValueType::Number(s.len() as f64));
                        Ok(())
                    }
                    _ => return Err(VmError::RuntimeError(VmRuntimeError::TypeMismatch)),
                }
            }
            _ => Err(VmError::RuntimeError(VmRuntimeError::UndefinedVariable(
                function.name().to_string(),
            ))),
        }
    }

    fn define_native_function(&mut self, name: &str, arity: usize) {
        let native_function = NativeFunction::new(name.to_string(), arity);
        let value = ValueType::NativeFunction(Box::new(native_function));
        self.globals.insert(name.to_string(), value);
    }

    fn ret(&mut self) -> Result<(), VmError> {
        let result = self.stack.pop()?;
        let frame = self
            .frames
            .pop()
            .ok_or(VmError::RuntimeError(VmRuntimeError::StackExhausted))?;
        self.stack.stack.truncate(frame.stack_top);
        self.stack.push(result);
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
        let mut vm = Vm {
            stack: VmStack::default(),
            frames: Vec::new(),
            globals: HashMap::new(),
            trace: Some(Box::new(tracer)),
            out: Rc::new(RefCell::new(out)),
        };
        vm.define_native_function("len", 1);
        vm
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

    pub fn get(&self, offset: usize) -> Option<&ValueType> {
        self.stack.get(offset)
    }

    fn peek(&self, offset: usize) -> Option<&ValueType> {
        self.stack.get(self.stack.len() - offset - 1)
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
    fn new(chunk: Chunk, stack_top: usize) -> Self {
        CallFrame {
            chunk,
            ip: 0,
            stack_top,
        }
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
