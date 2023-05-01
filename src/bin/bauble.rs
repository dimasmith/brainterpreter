use brainterpreter::interpret;
use clap::{command, Parser};
use env_logger::Builder;
use log::{debug, error, LevelFilter};
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(name = "bauble")]
#[command(about = "Interpret bauble source file")]
#[command(author, version, long_about = None)]
struct Args {
    /// The source file to run
    source: String,
    /// Enable trace output of the virtual machine.
    #[arg(long)]
    trace: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if args.trace {
        Builder::new().filter_level(LevelFilter::max()).init();
    } else {
        env_logger::init();
    }

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
