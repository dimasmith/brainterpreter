use std::error::Error;

use l9_vm::interpret;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    
    fun increment() {
        i = i + 1;
    }
    
    let i = 5;
    
    increment();
    increment();
    increment();
    print i;
    "#;
    interpret(source)
}
