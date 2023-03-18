//! Chunks of bytecode
use std::fmt::Display;

use crate::ops::Op;

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    ops: Vec<Op>,
}

impl Chunk {
    pub fn new(ops: Vec<Op>) -> Self {
        Chunk { ops }
    }

    pub fn push(mut self, op: Op) -> Self {
        self.ops.push(op);
        self
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
