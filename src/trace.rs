//! Instruments to trace virtual machine execution

use std::fmt::Debug;

use crate::vm::opcode::Chunk;
use crate::vm::VmStack;

pub trait VmStepTrace: Debug {
    // traces execution before opcode is processed
    fn trace_before(&self, ip: usize, chunk: &Chunk, stack: &VmStack);

    // traces execution after opcode is processed
    fn trace_after(&self, ip: usize, chunk: &Chunk, stack: &VmStack);
}
