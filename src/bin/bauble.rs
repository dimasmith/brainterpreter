use brainterpreter::interpret;
use clap::{command, Parser};
use log::{debug, error};
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(name = "bauble")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The source file to run
    source: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args = Args::parse();

    if let Err(e) = run(&args) {
        error!("{}", e);
    }

    Ok(())
}

fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let source = read_source_from_file(&args.source)?;
    interpret(&source)?;
    Ok(())
}

fn read_source_from_file(path: &str) -> Result<String, Box<dyn Error>> {
    debug!("running file: {}", path);
    let mut source = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut source)?;
    Ok(source)
}
