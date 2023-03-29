use std::fmt::Display;

/// Operations supported by the virtual machine
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    /// Print the top value of the stack.
    Return,
    /// Pushes floating-point constant on the stack.
    LoadFloat(f64),
    /// Add two top elements of the stack.
    Add,
    Sub,
    Mul,
    Div,
    /// Compares top values of the stack. Puts comparison result on top of the stack.
    Cmp,
    /// Negates value on top of the stack.
    Neg,
    /// Pushes true on the stack if the first value is less or equal to the second.
    Le,
    /// Pushes true on the stack if the first value is greater or equal to the second.
    Ge,
    /// Prints value on top of the stack.
    Print,
    /// Initialize global variable.
    Global(String),
    /// Load global variable value onto the stack.
    LoadGlobal(String),
    /// Pushes nil on the stack.
    Nil,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Return => write!(f, "RET"),
            Op::LoadFloat(n) => write!(f, "LD_F, {}", n),
            Op::Add => write!(f, "ADD"),
            Op::Sub => write!(f, "SUB"),
            Op::Mul => write!(f, "MUL"),
            Op::Div => write!(f, "DIV"),
            Op::Cmp => write!(f, "CMP"),
            Op::Le => write!(f, "LE"),
            Op::Ge => write!(f, "GE"),
            Op::Neg => write!(f, "NEG"),
            Op::Print => write!(f, "PRN"),
            Op::Global(name) => write!(f, "GLB, {}", name),
            Op::LoadGlobal(name) => write!(f, "LD_GL, {}", name),
            Op::Nil => write!(f, "NIL"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    ops: Vec<Op>,
}

impl Chunk {
    pub fn new(ops: Vec<Op>) -> Self {
        Chunk { ops }
    }

    pub fn push(mut self, op: Op) -> Self {
        self.add(op);
        self
    }

    pub fn add(&mut self, op: Op) {
        self.ops.push(op);
    }

    pub fn op(&self, idx: usize) -> Option<&Op> {
        self.ops.get(idx)
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
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
            .push(Op::LoadFloat(3.0))
            .push(Op::LoadFloat(4.0))
            .push(Op::Cmp)
            .push(Op::Return);

        assert_eq!(chunk.len(), 4);
    }
}
