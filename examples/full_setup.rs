use std::error::Error;

use brainterpreter::compiler::Compiler;
use brainterpreter::lexer::Lexer;
use brainterpreter::parser::Parser;
use brainterpreter::vm::Vm;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = "print 12 + 4;";
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;
    let mut compiler = Compiler::default();
    let chunk = compiler.compile_script(program)?;
    let mut vm = Vm::default();
    vm.run_script(chunk)?;

    Ok(())
}
