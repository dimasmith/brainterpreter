use std::fs::File;

use log::error;

use l9_vm::interpret;

fn main() {
    env_logger::init();
    let source_file = File::open("examples/brainfuck.l9").unwrap();
    let src = std::io::read_to_string(source_file).unwrap();
    match interpret(&src) {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    }
}
