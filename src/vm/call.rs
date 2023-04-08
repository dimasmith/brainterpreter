use crate::vm::opcode::{Chunk, Op};
use crate::vm::CallFrame;

impl CallFrame {
    pub fn new(chunk: Chunk, stack_top: usize) -> Self {
        CallFrame {
            chunk,
            ip: 0,
            stack_top,
        }
    }

    pub fn advance(&mut self) -> Option<&Op> {
        let op = self.chunk.op(self.ip);
        self.ip += 1;
        op
    }
}
