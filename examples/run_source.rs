use l9_vm::interpret;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    let a = 10;
    {
        let a = a;
        a = a + 1;
        print a;
    }
    print a;
    "#;
    interpret(source)
}
