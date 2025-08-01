use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::rc::Rc;

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use brainterpreter::compiler::Compiler;
use brainterpreter::lexer::Lexer;
use brainterpreter::parser::Parser;
use brainterpreter::vm::Vm;

fn interpret(source: &str) -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program()?;
    let mut compiler = Compiler::default();
    let script = compiler.compile(ast)?;
    let mut vm = Vm::with_io(Rc::new(RefCell::new(vec![])));
    vm.load_and_run(Rc::new(script))?;

    Ok(())
}

fn brainfuck_benchmark(c: &mut Criterion) {
    let source_file = File::open("benches/brainfuck.bbl").unwrap();
    let src = std::io::read_to_string(source_file).unwrap();
    c.bench_function("interpret", |b| b.iter(|| interpret(black_box(&src))));
}

fn parse_benchmark(c: &mut Criterion) {
    let source_file = File::open("benches/brainfuck.bbl").unwrap();
    let src = std::io::read_to_string(source_file).unwrap();
    let lexer = Lexer::new(&src);
    let mut parser = Parser::new(lexer);
    c.bench_function("parse", |b| b.iter(|| parser.parse_program()));

    let ast = parser.parse_program().unwrap();
    let mut compiler = Compiler::default();
    c.bench_function("compile", |b| b.iter(|| compiler.compile(ast.clone())));

    let ast = parser.parse_program().unwrap();
    let chunk = Rc::new(compiler.compile(ast).unwrap());
    let mut vm = Vm::with_io(Rc::new(RefCell::new(vec![])));
    c.bench_function("run", |b| b.iter(|| vm.load_and_run(chunk.clone())));
}

criterion_group!(benches, brainfuck_benchmark, parse_benchmark);
criterion_main!(benches);
