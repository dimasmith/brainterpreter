use std::error::Error;

use l9_vm::interpret;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = r#"
    fun factorial(n) {   
        if (n == 0) {
            return 1;
        }     
        return n * factorial(n - 1);
    }
    
    print factorial(4);    
    "#;
    interpret(source)
}
