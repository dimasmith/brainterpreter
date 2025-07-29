//! Logging facilities
use log::debug;

use crate::vm::exec::Chunk;
use crate::vm::trace::VmStepTrace;
use crate::vm::VmStack;

#[derive(Debug, Default)]
pub struct LoggingTracer;

impl VmStepTrace for LoggingTracer {
    fn trace_before(&self, ip: usize, chunk: &Chunk, _stack: &VmStack) {
        debug!("{}", "=".repeat(16));
        self.print_instructions_window(ip, chunk, 5);
        // self.print_stack(stack, "before");
    }

    fn trace_after(&self, _ip: usize, _chunk: &Chunk, stack: &VmStack) {
        self.print_stack(stack, "after");
    }
}

impl LoggingTracer {
    fn print_stack(&self, stack: &VmStack, stage: &str) {
        debug!("= stack {stage}");
        for i in 0..stack.len() {
            let value = stack.get(i).unwrap();
            debug!("{i}:\t{value}");
        }

        debug!("{}", "-".repeat(16));
    }

    fn print_instructions_window(&self, ip: usize, chunk: &Chunk, win_size: usize) {
        let win_size = std::cmp::min(chunk.ops_len(), win_size);
        let half_win = win_size / 2;
        let mut start_index = 0;
        if ip > half_win {
            start_index = ip - half_win;
        }
        let end_index = std::cmp::min(chunk.ops_len(), ip + 1);
        debug!("= instructions");
        for i in start_index..end_index {
            let op = chunk.op(i).unwrap();
            if i == ip {
                debug!("{i}:>\t{op}");
            } else {
                debug!("{i}:\t{op}");
            }
        }
        debug!("{}", "-".repeat(16));
    }
}
