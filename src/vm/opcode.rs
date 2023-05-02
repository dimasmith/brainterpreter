//! Operations supported by the virtual machine
use std::fmt::Display;

use crate::value::ValueType;

/// Operations supported by the virtual machine
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    /// Print the top value of the stack.
    Return,
    /// Call function stored in the top of the stack.
    Call(usize),
    /// Pushes floating-point constant on the stack.
    ConstFloat(f64),
    /// Pushes boolean constant on the stack.
    ConstBool(bool),
    /// Pushes constant from the constant pool on the stack.
    Const(usize),
    /// Loads indexed element from the array and pushes it on the stack.
    LoadIndex,
    StoreIndex,
    /// Add two top elements of the stack.
    Add,
    Sub,
    Mul,
    Div,
    /// Compares top values of the stack. Puts comparison result on top of the stack.
    Cmp,
    /// Inverts boolean value on top of the stack.
    Not,
    /// Pushes true on the stack if the first value is less or equal to the second.
    Le,
    /// Pushes true on the stack if the first value is greater or equal to the second.
    Ge,
    /// Prints value on top of the stack.
    Print,
    /// Takes the value from the top of the stack and stores it in the global variable.
    StoreGlobal(String),
    /// Load global variable value onto the stack.
    LoadGlobal(String),
    /// Takes the value from the top of the stack and stores it in the local variable.
    StoreLocal(usize),
    /// Load local variable value onto the stack.
    LoadLocal(usize),
    /// Pops value from the top of the stack.
    Pop,
    /// Pushes nil on the stack.
    Nil,
    /// Unconditional jump to the given offset.
    Jump(i32),
    /// Jump to the given offset if the top value of the stack is false.
    JumpIfFalse(i32),
    Array,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Const(idx) => write!(f, "CONST, {}", idx),
            Op::ConstFloat(n) => write!(f, "CONST_F, {}", n),
            Op::ConstBool(b) => write!(f, "CONST_B, {}", b),
            Op::Nil => write!(f, "CONST_NIL"),
            Op::Add => write!(f, "ADD"),
            Op::Sub => write!(f, "SUB"),
            Op::Mul => write!(f, "MUL"),
            Op::Div => write!(f, "DIV"),
            Op::Cmp => write!(f, "CMP"),
            Op::Le => write!(f, "LE"),
            Op::Ge => write!(f, "GE"),
            Op::Not => write!(f, "NEG"),
            Op::Print => write!(f, "PRN"),
            Op::LoadGlobal(name) => write!(f, "LD_G, {}", name),
            Op::StoreGlobal(name) => write!(f, "ST_G, {}", name),
            Op::LoadLocal(idx) => write!(f, "LD_L, {}", idx),
            Op::StoreLocal(idx) => write!(f, "ST_L, {}", idx),
            Op::Pop => write!(f, "POP"),
            Op::Return => write!(f, "RET"),
            Op::Call(arity) => write!(f, "CALL, {}", arity),
            Op::Jump(offset) => write!(f, "JMP, {}", offset),
            Op::JumpIfFalse(offset) => write!(f, "JZ, {}", offset),
            Op::LoadIndex => write!(f, "LD_IDX"),
            Op::StoreIndex => write!(f, "ST_IDX"),
            Op::Array => write!(f, "ARR"),
        }
    }
}

/// A chunk of virtual machine instructions and constants.
#[derive(Debug, Clone, Default)]
pub struct Chunk {
    constants: Vec<ValueType>,
    ops: Vec<Op>,
}

impl Chunk {
    pub fn new(ops: Vec<Op>) -> Self {
        Chunk {
            ops,
            constants: vec![],
        }
    }

    pub fn push(mut self, op: Op) -> Self {
        self.add_op(op);
        self
    }

    pub fn add_op(&mut self, op: Op) -> usize {
        self.ops.push(op);
        self.ops.len() - 1
    }

    pub fn add_constant(&mut self, value: ValueType) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn op(&self, idx: usize) -> Option<&Op> {
        self.ops.get(idx)
    }

    pub fn constant(&self, idx: usize) -> Option<&ValueType> {
        self.constants.get(idx)
    }

    pub fn constants(&self) -> &Vec<ValueType> {
        &self.constants
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<Op> {
        self.ops.iter()
    }

    pub fn last_index(&self) -> usize {
        self.ops.len() - 1
    }

    pub fn patch_jump(&mut self, address: usize, offset: i32) {
        if let Op::JumpIfFalse(_) = self.ops[address] {
            self.ops[address] = Op::JumpIfFalse(offset);
        } else if let Op::Jump(_) = self.ops[address] {
            self.ops[address] = Op::Jump(offset);
        } else {
            panic!("Invalid jump address");
        }
    }

    /// Directs jump instruction at jump_address to the target_address.
    pub fn patch_jump_to(&mut self, jump_address: usize, target_address: usize) {
        let offset = target_address as i32 - jump_address as i32;
        self.patch_jump(jump_address, offset);
    }

    /// Directs jump instruction at jump_address to the last instruction.
    pub fn patch_jump_to_last(&mut self, jump_address: usize) {
        self.patch_jump_to(jump_address, self.last_index());
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for op in self.ops.iter() {
            writeln!(f, "{}", op)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_program() {
        let chunk = Chunk::default()
            .push(Op::ConstFloat(3.0))
            .push(Op::ConstFloat(4.0))
            .push(Op::Cmp)
            .push(Op::Return);

        assert_eq!(chunk.len(), 4);
    }

    #[test]
    fn patch_conditional_jump() {
        let mut chunk = Chunk::default()
            .push(Op::ConstFloat(3.0))
            .push(Op::ConstFloat(4.0))
            .push(Op::Cmp);
        let jump_address = chunk.add_op(Op::JumpIfFalse(0));

        chunk.patch_jump(jump_address, -2);

        assert_eq!(chunk.op(jump_address), Some(&Op::JumpIfFalse(-2)));
    }

    #[test]
    fn patch_unconditional_jump() {
        let mut chunk = Chunk::default()
            .push(Op::ConstFloat(3.0))
            .push(Op::ConstFloat(4.0))
            .push(Op::Cmp);
        let jump_address = chunk.add_op(Op::Jump(0));

        chunk.patch_jump(jump_address, -1);

        assert_eq!(chunk.op(jump_address), Some(&Op::Jump(-1)));
    }

    #[test]
    #[should_panic]
    fn patch_jump_invalid_operation() {
        let mut chunk = Chunk::default()
            .push(Op::ConstFloat(3.0))
            .push(Op::ConstFloat(4.0))
            .push(Op::Cmp);
        let jump_address = chunk.add_op(Op::ConstFloat(0.0));

        chunk.patch_jump(jump_address, -1);
    }

    #[test]
    fn jump_to() {
        let mut chunk = Chunk::default();
        let target_address = chunk.add_op(Op::ConstFloat(3.0));
        chunk.add_op(Op::ConstFloat(4.0));
        chunk.add_op(Op::Cmp);
        let jump_address = chunk.add_op(Op::Jump(0));

        chunk.patch_jump_to(jump_address, target_address);

        assert_eq!(chunk.op(jump_address), Some(&Op::Jump(-3)));
    }
}
