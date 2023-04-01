use l9_vm::interpret;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    let a = 5;
    {
        let a = 10;        
        let a = 15;
        print a;
    }
    print a;
    "#;
    interpret(source)
}
