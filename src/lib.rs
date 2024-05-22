//! Interpreter for Bauble programming language
use std::error::Error;
use std::rc::Rc;

use vm::Vm;

use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub mod ast;
pub mod compiler;
pub mod lexer;
pub mod log;
pub mod parser;
pub mod source;
pub mod value;
pub mod vm;

/// Shortcut function to interpret the source code.
pub fn interpret(source: &str) -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program()?;
    let mut compiler = Compiler::default();
    let chunk = compiler.compile(ast)?;
    let mut vm = Vm::default();
    vm.load_and_run(Rc::new(chunk))?;

    Ok(())
}
