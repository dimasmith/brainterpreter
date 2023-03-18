//! Set of operations supported by the virtual machine
use std::fmt::Display;

/// Operations supported by the virtual machine
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Return,
    LoadFloat(f64),
    Add,
    Cmp,
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
