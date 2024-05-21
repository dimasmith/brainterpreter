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
        let i = self
            .constants
            .iter()
            .enumerate()
            .find(|(_, v)| **v == value)
            .map(|(i, _)| i);
        if let Some(idx) = i {
            return idx;
        }
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

    #[test]
    fn reuse_constant_pool_entries() {
        let mut chunk = Chunk::default();
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
