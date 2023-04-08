use log::error;

use l9_vm::interpret;

fn main() {
    env_logger::init();
    let source = r#"
    let memory = [0;10];
    print memory[0];
    "#;
    match interpret(source) {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    }
}
