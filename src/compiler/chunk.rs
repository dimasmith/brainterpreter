//! Helps to build executable chunks.

use log::trace;

use crate::{
    value::ValueType,
    vm::{exec::Chunk, opcode::Op},
};

/// Gradually builds executable chunks.
#[derive(Debug, Clone, Default)]
pub struct ChunkBuilder {
    constants: Vec<ValueType>,
    ops: Vec<Op>,
}

impl ChunkBuilder {
    /// Adds new operation to the program.
    /// Returns the address of the op in the program.
    pub fn add_op(&mut self, op: Op) -> usize {
        self.ops.push(op);
        self.ops.len() - 1
    }

    /// Returns the address of a last op that was added to the chunk.
    pub fn last_op_address(&self) -> usize {
        self.ops.len() - 1
    }

    /// Adds constant to constants table.
    /// If the constant is already in the table, the method does not add it again and instead
    /// returns the index of existing constant.
    pub fn add_constant(&mut self, value: ValueType) -> usize {
        if let Some((i, v)) = self
            .constants
            .iter()
            .enumerate()
            .find(|(_, v)| *v == &value)
        {
            trace!("found constant {:?} on index {}", v, i);
            i
        } else {
            self.constants.push(value);
            self.constants.len() - 1
        }
    }

    /// Sets target address to previously added jump instruction.
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
        self.patch_jump_to(jump_address, self.last_op_address());
    }

    /// Produces a [Chunk] from the builder.
    pub fn build(self) -> Chunk {
        Chunk::new(self.ops, self.constants)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chunk_builder_from_ops(ops: impl IntoIterator<Item = Op>) -> ChunkBuilder {
        let mut chunk_builder = ChunkBuilder::default();
        for op in ops {
            chunk_builder.add_op(op);
        }
        chunk_builder
    }

    #[test]
    fn patch_conditional_jump() {
        let mut chunk_builder = ChunkBuilder::default();
        chunk_builder.add_op(Op::ConstFloat(3.0));
        chunk_builder.add_op(Op::ConstFloat(4.0));
        chunk_builder.add_op(Op::Cmp);
        let jump_address = chunk_builder.add_op(Op::JumpIfFalse(0));
        chunk_builder.patch_jump(jump_address, -2);

        let chunk = chunk_builder.build();

        assert_eq!(chunk.op(jump_address), Some(&Op::JumpIfFalse(-2)));
    }

    #[test]
    fn patch_unconditional_jump() {
        let mut chunk_builder =
            chunk_builder_from_ops([Op::ConstFloat(3.0), Op::ConstFloat(4.0), Op::Cmp]);
        let jump_address = chunk_builder.add_op(Op::Jump(0));

        chunk_builder.patch_jump(jump_address, -1);

        let chunk = chunk_builder.build();
        assert_eq!(chunk.op(jump_address), Some(&Op::Jump(-1)));
    }

    #[test]
    #[should_panic]
    fn patch_jump_invalid_operation() {
        let mut chunk_builder =
            chunk_builder_from_ops([Op::ConstFloat(3.0), Op::ConstFloat(4.0), Op::Cmp]);
        let jump_address = chunk_builder.add_op(Op::ConstFloat(0.0));

        chunk_builder.patch_jump(jump_address, -1);
    }

    #[test]
    fn jump_to() {
        let mut chunk_builder = ChunkBuilder::default();
        let target_address = chunk_builder.add_op(Op::ConstFloat(3.0));
        chunk_builder.add_op(Op::ConstFloat(4.0));
        chunk_builder.add_op(Op::Cmp);
        let jump_address = chunk_builder.add_op(Op::Jump(0));

        chunk_builder.patch_jump_to(jump_address, target_address);

        let chunk = chunk_builder.build();
        assert_eq!(chunk.op(jump_address), Some(&Op::Jump(-3)));
    }

    #[test]
    fn reuse_constant_pool_entries() {
        let mut chunk = ChunkBuilder::default();
        let foo_index = chunk.add_constant(ValueType::string("foo"));
        let bar_index = chunk.add_constant(ValueType::string("bar"));
        let duplicate_index = chunk.add_constant(ValueType::string("foo"));

        assert_eq!(
            foo_index, duplicate_index,
            "constant pool put duplicate entry in a separate constant pool entry"
        );
        assert_ne!(
            foo_index, bar_index,
            "constant pool put different constants in the same entry"
        );
    }
}
