//! Instruments to trace virtual machine execution

use crate::chunk::Chunk;
use crate::vm::VmStack;
use std::fmt::Debug;

pub trait VmStepTrace: Debug {
    fn trace(&self, ip: usize, chunk: &Chunk, stack: &VmStack);
}
