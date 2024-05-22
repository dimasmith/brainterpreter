//! Defines in-memory format of the executable VM can run.
//!
//! The Chunk is an executable which can be run via the virtual machine.

use std::fmt::Display;

use crate::value::ValueType;

use super::opcode::Op;

/// In-memory representation of the executable VM can run.
///
/// The executable chunk holds two main areas:
/// - Instructions - a list of VM operations.
/// - Constant pool - a list of constants necessary for program execution.
///
/// The Chunk is generally immutable.
/// The compiler uses [ChunkBuilder](crate::compiler::chunk::ChunkBuilder) to gradually build executable chunks.
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

    /// Returns operation on address.
    pub fn op(&self, idx: usize) -> Option<&Op> {
        self.ops.get(idx)
    }

    /// Count of opcodes in executable chunk.
    pub fn ops_len(&self) -> usize {
        self.ops.len()
    }

    /// Returns iterator of opcode references.
    pub fn ops(&self) -> impl ExactSizeIterator<Item = &Op> {
        self.ops.iter()
    }

    /// Get constant from a constants pool by index.
    pub fn constant(&self, idx: usize) -> Option<&ValueType> {
        self.constants.get(idx)
    }

    /// Return iterator over constants in constants pool.
    pub fn constants(&self) -> impl ExactSizeIterator<Item = &ValueType> {
        self.constants.iter()
    }

    /// Count of constants in executable chunk.
    pub fn constants_len(&self) -> usize {
        self.constants.len()
    }

    /// Returns true if the chunk has no opcodes.
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
