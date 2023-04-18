use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::rc::Rc;

use brainterpreter::compiler::Compiler;
use brainterpreter::lexer::Lexer;
use brainterpreter::parser::Parser;
use brainterpreter::vm::Vm;

#[test]
fn brainfuck_interpreter() {
    env_logger::init();
    let source_file = File::open("examples/brainfuck.l9").unwrap();
    let src = std::io::read_to_string(source_file).unwrap();
    let io = interpret(&src).unwrap();
    let out = String::from_utf8(io).unwrap();
    let mut o = String::new();
    for c in out.chars() {
        if c != '\n' {
            o.push(c);
        }
    }

    assert_eq!(o, "Hello World!");
}

pub fn interpret(source: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let io = Rc::new(RefCell::new(vec![]));
    {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_program()?;
        let mut compiler = Compiler::default();
        let script = compiler.compile_script(ast)?;
        let mut vm = Vm::with_io(io.clone());
        vm.run_script(script)?;
    }

    let output = io.borrow();
    Ok(output.clone())
}
