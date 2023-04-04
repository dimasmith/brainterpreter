use std::error::Error;

use l9_vm::interpret;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    fun inc() {
        return "Hello";
    }
    print inc() + ", world!";    
    "#;
    interpret(source)
}
