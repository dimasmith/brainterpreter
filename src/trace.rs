//! Instruments to trace virtual machine execution

use crate::chunk::Chunk;
use crate::vm::VmStack;
use std::fmt::Debug;

pub trait VmStepTrace: Debug {
    // traces execution before opcode is processed
    fn trace_before(&self, ip: usize, chunk: &Chunk, stack: &VmStack);

    // traces execution after opcode is processed
    fn trace_after(&self, ip: usize, chunk: &Chunk, stack: &VmStack);
}
