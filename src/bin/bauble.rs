use brainterpreter::compiler::Compiler;
use brainterpreter::interpret;
use brainterpreter::lexer::Lexer;
use brainterpreter::parser::Parser as BaubleParser;
use brainterpreter::vm::disassembler::disassemble;
use clap::{Parser, Subcommand};
use env_logger::Builder;
use log::{debug, error, LevelFilter};
use std::error::Error;
use std::fs::File;
use std::io::{stdout, Read};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "bauble")]
#[command(about = "Interpret bauble source file")]
#[command(author, version, long_about = None)]
struct Args {
    /// Enable trace output of the virtual machine.
    #[arg(long)]
    trace: bool,
    #[command(subcommand)]
    command: Commands,
    /// The source file to run
    source_path: PathBuf,
}

#[derive(Subcommand, Debug, Default)]
enum Commands {
    /// Create assembly file instead of running a program
    Disassemble,
    /// Runs the program from the source file
    #[default]
    Run,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if args.trace {
        Builder::new().filter_level(LevelFilter::max()).init();
    } else {
        env_logger::init();
    }

    let result = match args.command {
        Commands::Disassemble => disassemble_file(&args),
        Commands::Run => run(&args),
    };

    if let Err(e) = result {
        error!("{}", e);
    }

    Ok(())
}

fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let source = read_source_from_file(&args.source_path)?;
    interpret(&source)?;
    Ok(())
}

fn read_source_from_file(path: &Path) -> Result<String, Box<dyn Error>> {
    debug!("running file: {}", path.display());
    let mut source = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut source)?;
    Ok(source)
}

fn disassemble_file(args: &Args) -> Result<(), Box<dyn Error>> {
    let source = read_source_from_file(&args.source_path)?;
    let lexer = Lexer::new(&source);
    let mut parser = BaubleParser::new(lexer);
    let ast = parser.parse_program()?;
    let mut compiler = Compiler::default();
    let chunk = compiler.compile(ast)?;
    disassemble(&chunk, stdout())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }
}
