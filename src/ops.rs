//! Set of operations supported by the virtual machine
use std::fmt::Display;

/// Operations supported by the virtual machine
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    /// Print the top value of the stack.
    Return,
    /// Pushes floating-point constant on the stack.
    LoadFloat(f64),
    /// Add two top elements of the stack.
    Add,
    /// Compares top values of the stack. Puts comparison result on top of the stack.
    Cmp,
    /// Negates value on top of the stack.
    Neg,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Return => write!(f, "RET"),
            Op::LoadFloat(n) => write!(f, "LD_F, {}", n),
            Op::Add => write!(f, "ADD"),
            Op::Cmp => write!(f, "CMP"),
            Op::Neg => write!(f, "NEG"),
        }
    }
}
