//! Logging facilities
use log::debug;

use crate::trace::VmStepTrace;
use crate::vm::opcode::Chunk;
use crate::vm::VmStack;

#[derive(Debug, Default)]
pub struct LoggingTracer;

impl VmStepTrace for LoggingTracer {
    fn trace_before(&self, ip: usize, chunk: &Chunk, stack: &VmStack) {
        debug!("{}", "=".repeat(16));
        self.print_instructions_window(ip, chunk, 5);
        self.print_stack(stack, "before");
    }

    fn trace_after(&self, _ip: usize, _chunk: &Chunk, stack: &VmStack) {
        self.print_stack(stack, "after");
    }
}

impl LoggingTracer {
    fn print_stack(&self, stack: &VmStack, stage: &str) {
        debug!("= stack {}", stage);
        for i in 0..stack.len() {
            let value = stack.get(i).unwrap();
            debug!("{}:\t{}", i, value);
        }

        debug!("{}", "-".repeat(16));
    }

    fn print_instructions_window(&self, ip: usize, chunk: &Chunk, win_size: usize) {
        let win_size = std::cmp::min(chunk.len(), win_size);
        let half_win = win_size / 2;
        let mut start_index = 0;
        if ip > half_win {
            start_index = ip - half_win;
        }
        let end_index = std::cmp::min(chunk.len(), ip + 1);
        debug!("= instructions");
        for i in start_index..end_index {
            let op = chunk.op(i).unwrap();
            if i == ip {
                debug!("{}:>\t{}", i, op);
            } else {
                debug!("{}:\t{}", i, op);
            }
        }
        debug!("{}", "-".repeat(16));
    }
}
