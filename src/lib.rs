//! Virtual machine to support running l9 toy programming language
use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::Vm;
use std::error::Error;

pub mod ast;
pub mod chunk;
pub mod compiler;
pub mod dbg;
pub mod lexer;
pub mod log;
pub mod ops;
pub mod parser;
pub mod trace;
pub mod value;
pub mod vm;

/// Shortcut function to interpret the source code.
pub fn interpret(source: &str) -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse()?;
    let mut compiler = Compiler::default();
    let chunk = compiler.compile(&ast);
    let mut vm = Vm::default();
    vm.interpret(chunk)?;

    Ok(())
}
