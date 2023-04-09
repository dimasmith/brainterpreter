use log::error;

use l9_vm::interpret;

fn main() {
    env_logger::init();
    let source = r#"
    let i = 2;
    while (i > 0) {
        // print i;
        i = i - 1;
    }
    print 100;
    "#;
    match interpret(source) {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    }
}
