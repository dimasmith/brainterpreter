use l9_vm::interpret;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
        let a = !true;
        print a;
    "#;
    interpret(source)
}
