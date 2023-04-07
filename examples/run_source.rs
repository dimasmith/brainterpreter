use std::error::Error;

use log::error;

use l9_vm::interpret;

fn main() {
    env_logger::init();
    let source = r#"
    {
        let a = 3;
    }
    "#;
    match interpret(source) {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    }
}
