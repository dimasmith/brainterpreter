use std::fs::File;

use log::error;

use brainterpreter::interpret;

fn main() {
    env_logger::init();
    let source_file = File::open("examples/brainfuck.bbl").unwrap();
    let src = std::io::read_to_string(source_file).unwrap();
    match interpret(&src) {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    }
}
