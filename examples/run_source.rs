use std::error::Error;

use l9_vm::interpret;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    print "ab"[1] == "a";      
    "#;
    interpret(source)
}
