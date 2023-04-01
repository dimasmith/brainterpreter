use std::error::Error;

use l9_vm::interpret;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    {
        let a = 2;
        if (a == 1) {
            print 0;
        } else if (a == 2) {
            print 1;
        } else {
            print 2;
        }
    }
    "#;
    interpret(source)
}
