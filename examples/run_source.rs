use std::error::Error;

use l9_vm::interpret;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    {
        let a = 4;
        if (a < 5) {
            a = a * 2;
        }
        print a;
    }
    "#;
    interpret(source)
}
