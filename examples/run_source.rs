use std::error::Error;

use log::error;

use l9_vm::interpret;

fn main() {
    env_logger::init();
    let source = r#"
    fun add(a, b) {
        return a + b;
    }
    print add(1, 2);
    "#;
    match interpret(source) {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    }
}
