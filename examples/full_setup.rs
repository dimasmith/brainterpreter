use std::error::Error;
use std::rc::Rc;

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
    let chunk = compiler.compile(program)?;
    let mut vm = Vm::default();
    vm.load_and_run(Rc::new(chunk))?;

    Ok(())
}
