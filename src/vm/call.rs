use crate::vm::opcode::{Chunk, Op};

#[derive(Debug)]
pub struct CallFrame {
    ip: usize,
    chunk: Chunk,
    stack_top: usize,
}

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

    pub fn stack_top(&self) -> usize {
        self.stack_top
    }

    pub fn ip(&self) -> usize {
        self.ip
    }

    pub fn jump_to(&mut self, ip: usize) {
        self.ip = ip;
    }

    pub fn chunk(&self) -> Chunk {
        self.chunk.clone()
    }
}
