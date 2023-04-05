use std::error::Error;

use l9_vm::interpret;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    let i = 0;
    fun recurse() {
        if (i < 5) {
            i = i + 1;
            recurse();
        }
        return i;
    }
    
    print recurse();
    "#;
    interpret(source)
}
