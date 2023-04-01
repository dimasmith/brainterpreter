use std::error::Error;

use l9_vm::interpret;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    {
        let a = 1;  
        let b = 2;
        a = a + b;
        print a;
    }
    "#;
    interpret(source)
}
