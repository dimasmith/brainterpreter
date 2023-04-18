use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::rc::Rc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use brainterpreter::compiler::Compiler;
use brainterpreter::lexer::Lexer;
use brainterpreter::parser::Parser;
use brainterpreter::vm::Vm;

fn interpret(source: &str) -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program()?;
    let mut compiler = Compiler::default();
    let script = compiler.compile_script(ast)?;
    let mut vm = Vm::with_io(Rc::new(RefCell::new(vec![])));
    vm.run_script(script)?;

    Ok(())
}

fn brainfuck_benchmark(c: &mut Criterion) {
    let source_file = File::open("benches/brainfuck.l9").unwrap();
    let src = std::io::read_to_string(source_file).unwrap();
    c.bench_function("brainfuck", |b| b.iter(|| interpret(black_box(&src))));
}

criterion_group!(benches, brainfuck_benchmark);
criterion_main!(benches);
