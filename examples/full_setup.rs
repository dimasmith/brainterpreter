use l9_vm::compiler::Compiler;
use l9_vm::lexer::Lexer;
use l9_vm::parser::Parser;
use l9_vm::vm::Vm;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let source = "print 12 + 4;";
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program()?;
    let mut compiler = Compiler::default();
    let chunk = compiler.compile_program(ast);
    let mut vm = Vm::default();
    vm.interpret(chunk)?;

    Ok(())
}
