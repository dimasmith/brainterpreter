use std::error::Error;

use log::error;

use l9_vm::interpret;

fn main() {
    env_logger::init();
    let source = r#"
    let word = "Rust";    
    let i = 0;
    while (i < len(word)) {
        print word[i];
        i = i + 1;
    }
    "#;
    match interpret(source) {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    }
}
