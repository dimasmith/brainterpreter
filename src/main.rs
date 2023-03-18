use l9_vm::chunk::Chunk;
use l9_vm::ops::Op;
use l9_vm::vm::{Vm, VmError};

fn main() -> Result<(), VmError> {
    env_logger::init();

    // run smallest program
    let mut vm = Vm::default();
    let program = Chunk::new(vec![
        Op::LoadFloat(3.0),
        Op::LoadFloat(4.0),
        Op::Add,
        Op::LoadFloat(7.0),
        Op::Cmp,
        //        Op::Neg, // Runtime error happens here
        Op::Return,
    ]);
    vm.interpret(program)?;
    Ok(())
}
