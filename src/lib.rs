//! Virtual machine to support running l9 toy programming language
use std::error::Error;

use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::vm::Vm;

pub mod ast;
pub mod compiler;
pub mod lexer;
pub mod log;
pub mod parser;
pub mod source;
pub mod trace;
pub mod value;
pub mod vm;

/// Shortcut function to interpret the source code.
pub fn interpret(source: &str) -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program()?;
    let mut compiler = Compiler::default();
    let script = compiler.compile_script(ast)?;
    let mut vm = Vm::default();
    vm.run_script(script)?;

    Ok(())
}
