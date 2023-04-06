use std::error::Error;

use l9_vm::interpret;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    let i = 5;
    i = 2;
    print i;   
    "#;
    interpret(source)
}
