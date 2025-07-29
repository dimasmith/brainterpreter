use log::error;

use brainterpreter::interpret;

fn main() {
    env_logger::init();
    let source = r#"
    fun check() {
        print "CHK";
    }
    "#;
    match interpret(source) {
        Ok(_) => {}
        Err(e) => {
            error!("{e}");
        }
    }
}
