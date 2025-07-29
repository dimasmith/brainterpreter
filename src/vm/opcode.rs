//! Operations supported by the virtual machine
use std::fmt::Display;

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
    /// The name of the variable is taken from the constant pool.
    StoreGlobal(usize),
    /// Load global variable value onto the stack.
    /// The name of the variable is taken from the constant pool.
    LoadGlobal(usize),
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
            Op::Const(idx) => write!(f, "CONST, {idx}"),
            Op::ConstFloat(n) => write!(f, "CONST_F, {n}"),
            Op::ConstBool(b) => write!(f, "CONST_B, {b}"),
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
            Op::LoadGlobal(idx) => write!(f, "LD_G, {idx}"),
            Op::StoreGlobal(idx) => write!(f, "ST_G, {idx}"),
            Op::LoadLocal(idx) => write!(f, "LD_L, {idx}"),
            Op::StoreLocal(idx) => write!(f, "ST_L, {idx}"),
            Op::Pop => write!(f, "POP"),
            Op::Return => write!(f, "RET"),
            Op::Call(arity) => write!(f, "CALL, {arity}"),
            Op::Jump(offset) => write!(f, "JMP, {offset}"),
            Op::JumpIfFalse(offset) => write!(f, "JZ, {offset}"),
            Op::LoadIndex => write!(f, "LD_IDX"),
            Op::StoreIndex => write!(f, "ST_IDX"),
            Op::Array => write!(f, "ARR"),
        }
    }
}
