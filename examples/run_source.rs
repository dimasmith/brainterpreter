use std::error::Error;

use log::error;

use l9_vm::interpret;

fn main() {
    env_logger::init();
    let source = r#"
    let w = "Rust";    
    w[0] = "D";
    print w;
    "#;
    match interpret(source) {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    }
}
