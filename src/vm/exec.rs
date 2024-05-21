//! Defines in-memory format of the executable VM can run.

use std::fmt::Display;

use crate::value::ValueType;

use super::opcode::Op;
/// In-memory representation of the executable VM can run.
///
/// The executable chunk holds two main areas:
/// - Instructions - a list of VM operations.
/// - Constant pool - a list of constants necessary for program execution.
#[derive(Debug, Clone, Default)]
pub struct Chunk {
    constants: Vec<ValueType>,
    ops: Vec<Op>,
}

impl Chunk {
    /// Creates a new chunk from a list of operations and constants.
    pub fn new<I, C>(ops: I, constants: C) -> Self
    where
        I: IntoIterator<Item = Op>,
        C: IntoIterator<Item = ValueType>,
    {
        Chunk {
            ops: ops.into_iter().collect(),
            constants: constants.into_iter().collect(),
        }
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
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for op in self.ops.iter() {
            writeln!(f, "{}", op)?
        }
        Ok(())
    }
}
