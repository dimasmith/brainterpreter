use l9_vm::interpret;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
        let n = 7 + 5 == 3 * 4;        
        print n;
    "#;
    interpret(source)
}
