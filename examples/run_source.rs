use l9_vm::interpret;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
        let a;
        let b = 10;
        a = 5;
    "#;
    interpret(source)
}
