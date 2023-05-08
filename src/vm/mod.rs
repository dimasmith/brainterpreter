//! Virtual machine for executing bytecode

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::rc::Rc;

use thiserror::Error;

use call::CallFrame;

use crate::log::LoggingTracer;
use crate::value::{Function, NativeFunction, TypeError, ValueType};
use crate::vm::native::std_lib;
use crate::vm::opcode::{Chunk, Op};
use crate::vm::trace::VmStepTrace;

mod call;
pub mod disassembler;
mod native;
pub mod opcode;
mod stack;
pub mod trace;

type VmResult = Result<(), VmRuntimeError>;

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

/// Virtual machine to run programs
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

impl Vm {
    pub fn run_script(&mut self, script: Function) -> VmResult {
        let call_frame = CallFrame::new(script.chunk().clone(), 0);
        self.frames.push(call_frame);
        self.stack.push(ValueType::Function(Box::new(script)));
        self.run()?;
        self.stack.pop()?;
        Ok(())
    }

    fn run(&mut self) -> VmResult {
        while let Some(op) = self.advance() {
            let op = op.clone();
            self.trace_before();
            match op {
                Op::Return => self.ret()?,
                Op::Array => self.initialize_array()?,
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
                Op::StoreGlobal(idx) => self.store_global(idx)?,
                Op::LoadGlobal(idx) => self.load_global(idx)?,
                Op::StoreLocal(offset) => self.store_local(offset)?,
                Op::LoadLocal(offset) => self.load_local(offset)?,
                Op::Jump(offset) => self.jump(offset)?,
                Op::JumpIfFalse(offset) => self.jump_if_false(offset)?,
            }
            self.trace_after()
        }
        Ok(())
    }

    fn binary_operation(&mut self, operation: Op) -> VmResult {
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
            (Op::LoadIndex, _, _) => self.load_index(&value_a, &value_b)?,
            (Op::Not, _, _) => {
                return Err(VmRuntimeError::WrongOperation);
            }
            _ => {
                return Err(VmRuntimeError::TypeMismatch);
            }
        };
        self.stack.push(result);
        Ok(())
    }

    fn load_index(
        &mut self,
        value_a: &ValueType,
        value_b: &ValueType,
    ) -> Result<ValueType, VmRuntimeError> {
        value_a
            .get(value_b)
            .map_err(VmRuntimeError::ArrayAccessError)
    }

    fn store_index(&mut self) -> VmResult {
        let value = self.stack.pop()?;
        let target = self.stack.pop()?;
        let idx = self.stack.pop()?;
        let new_value = target
            .set(&idx, value)
            .map_err(VmRuntimeError::ArrayAccessError)?;
        self.stack.push(new_value);
        Ok(())
    }

    fn not(&mut self) -> VmResult {
        let result = match self.stack.pop()? {
            ValueType::Bool(b) => ValueType::Bool(!b),
            _ => {
                return Err(VmRuntimeError::TypeMismatch);
            }
        };
        self.stack.push(result);
        Ok(())
    }

    fn print(&mut self) -> VmResult {
        let line = match self.stack.pop()? {
            ValueType::Number(n) => n.to_string(),
            ValueType::Bool(b) => b.to_string(),
            ValueType::Address(a) => a.to_string(),
            ValueType::Nil => "nil".to_string(),
            ValueType::Function(f) => {
                format!("{}:{}", "fun", f.name())
            }
            ValueType::NativeFunction(f) => {
                format!("[{}]:{}", "fun", f.name())
            }
            ValueType::Text(s) => *s,
            ValueType::Array(a) => format_args!("[{}]\n", a.len()).to_string(),
            ValueType::ArrayRef(a) => format_args!("&[{}]\n", a.borrow().len()).to_string(),
        };
        self.out
            .borrow_mut()
            .write_fmt(format_args!("{}\n", line))
            .map_err(VmRuntimeError::IoError)
    }

    fn constant_entry(&self, idx: usize) -> Result<&ValueType, VmRuntimeError> {
        let value = self
            .chunk()
            .constant(idx)
            .ok_or(VmRuntimeError::UndefinedConstant(idx))?;
        Ok(value)
    }

    fn variable_name(&self, idx: usize) -> Result<String, VmRuntimeError> {
        let value = self.constant_entry(idx)?;
        if let ValueType::Text(name) = value {
            Ok(name.to_string())
        } else {
            Err(VmRuntimeError::TypeMismatch)
        }
    }

    fn store_global(&mut self, idx: usize) -> VmResult {
        let name = self.variable_name(idx)?;
        let value = self.stack.peek(0).ok_or(VmRuntimeError::StackExhausted)?;
        self.globals.insert(name, value.clone());
        Ok(())
    }

    fn load_global(&mut self, idx: usize) -> VmResult {
        let name = self.variable_name(idx)?;
        let value = self
            .globals
            .get(&name)
            .ok_or(VmRuntimeError::UndefinedVariable(name.clone()))?;
        self.stack.push(value.clone());
        Ok(())
    }

    fn store_local(&mut self, offset: usize) -> VmResult {
        let value = self.stack.last().ok_or(VmRuntimeError::StackExhausted)?;
        let frame_offset = self.frames.last().unwrap().stack_top() + offset + 1;
        self.stack.set(frame_offset, value.clone())?;
        Ok(())
    }

    fn load_local(&mut self, offset: usize) -> VmResult {
        let frame_offset = self.frames.last().unwrap().stack_top() + offset + 1;
        let value = self
            .stack
            .stack
            .get(frame_offset)
            .ok_or(VmRuntimeError::UndefinedVariable(frame_offset.to_string()))?;
        self.stack.push(value.clone());
        Ok(())
    }

    fn jump(&mut self, offset: i32) -> VmResult {
        self.offset_ip(offset as isize)?;
        Ok(())
    }

    fn jump_if_false(&mut self, offset: i32) -> VmResult {
        let value = self.stack.pop()?;
        if let ValueType::Bool(b) = value {
            if !b {
                self.offset_ip(offset as isize)?;
            }
        } else {
            return Err(VmRuntimeError::TypeMismatch);
        }
        Ok(())
    }

    fn call(&mut self, arity: usize) -> VmResult {
        let value = self.peek_value(arity)?.clone();
        match &value {
            ValueType::Function(f) => self.call_function(f, arity),
            ValueType::NativeFunction(f) => self.call_native_function(f, arity),
            _ => Err(VmRuntimeError::TypeMismatch),
        }
    }

    fn initialize_array(&mut self) -> VmResult {
        let initial_value = self.stack.pop()?;
        let size = self.index()?;
        let mut array = vec![];
        array.resize(size, initial_value);
        // self.stack.push(ValueType::Array(Box::new(array)));
        self.stack
            .push(ValueType::ArrayRef(Rc::new(RefCell::new(array))));
        Ok(())
    }

    fn peek_value(&mut self, arity: usize) -> Result<&ValueType, VmRuntimeError> {
        self.stack.peek(arity).ok_or(VmRuntimeError::StackExhausted)
    }

    fn call_function(&mut self, function: &Function, arity: usize) -> VmResult {
        if arity != function.arity() {
            return Err(VmRuntimeError::TypeMismatch);
        }
        let stack_top = self.stack.len() - function.arity() - 1;
        let frame = CallFrame::new(function.chunk().clone(), stack_top);
        self.frames.push(frame);
        Ok(())
    }

    fn call_native_function(&mut self, function: &NativeFunction, arity: usize) -> VmResult {
        if arity != function.arity() {
            return Err(VmRuntimeError::TypeMismatch);
        }
        function.call(self)
    }

    fn define_native_function(&mut self, native_function: NativeFunction) {
        let name = native_function.name().to_string();
        let value = ValueType::NativeFunction(Rc::new(native_function));
        self.globals.insert(name, value);
    }

    fn ret(&mut self) -> VmResult {
        let result = self.stack.pop()?;
        let frame = self.frames.pop().ok_or(VmRuntimeError::StackExhausted)?;
        self.stack.stack.truncate(frame.stack_top());
        self.stack.push(result);
        Ok(())
    }

    fn offset_ip(&mut self, offset: isize) -> VmResult {
        let frame = self.frames.last_mut().unwrap();
        frame
            .jump(offset)
            .map_err(|_| VmRuntimeError::IllegalJump(frame.ip(), offset))
    }

    fn advance(&mut self) -> Option<&Op> {
        self.frames.last_mut().and_then(|frame| frame.advance())
    }

    fn ip(&self) -> usize {
        self.frames.last().map(|frame| frame.ip()).unwrap_or(0)
    }

    fn chunk(&self) -> &Chunk {
        let frame = self.frames.last().unwrap();
        frame.chunk()
    }

    fn trace_before(&self) {
        if let Some(ref tracer) = self.trace {
            tracer.trace_before(self.ip() - 1, self.chunk(), &self.stack);
        }
    }

    fn trace_after(&mut self) {
        if let Some(trace) = &self.trace {
            trace.trace_after(self.ip(), self.chunk(), &self.stack);
        }
    }

    fn constant(&self, index: usize) -> Result<ValueType, VmRuntimeError> {
        let chunk = self.chunk();
        chunk
            .constant(index)
            .cloned()
            .ok_or(VmRuntimeError::UndefinedConstant(index))
    }
    fn index(&mut self) -> Result<usize, VmRuntimeError> {
        let value = self.stack.pop()?;
        if let ValueType::Number(n) = value {
            Ok(n as usize)
        } else {
            Err(VmRuntimeError::TypeMismatch)
        }
    }

    pub fn pop(&mut self) -> Result<ValueType, VmRuntimeError> {
        self.stack.pop()
    }

    pub fn push(&mut self, value: ValueType) {
        self.stack.push(value);
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
        std_lib()
            .iter()
            .for_each(|f| vm.define_native_function(f.clone()));
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
