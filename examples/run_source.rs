use std::error::Error;

use l9_vm::interpret;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    let i = 5;
    while (i > 0) {
        print i;
        i = i - 1;
    }
    print 100;
    "#;
    interpret(source)
}
