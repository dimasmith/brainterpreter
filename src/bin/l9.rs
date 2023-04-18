use brainterpreter::interpret;
use log::{debug, error};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let source = match env::args().nth(1) {
        Some(path) => read_source_from_file(&path)?,
        None => read_source_from_standard_input()?,
    };
    if let Err(e) = interpret(&source) {
        error!("{}", e);
    }
    Ok(())
}

fn read_source_from_file(path: &str) -> Result<String, Box<dyn Error>> {
    debug!("running file: {}", path);
    let mut source = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut source)?;
    Ok(source)
}

fn read_source_from_standard_input() -> Result<String, Box<dyn Error>> {
    debug!("running from standard input");
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source)?;
    Ok(source)
}
