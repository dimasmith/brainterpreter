use l9_vm::interpret;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    interpret("15 - 5 + 10")
}
