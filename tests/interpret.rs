use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use l9_vm::compiler::Compiler;
use l9_vm::lexer::Lexer;
use l9_vm::parser::Parser;
use l9_vm::vm::vm::Vm;

#[test]
fn expression_with_negative_numbers() {
    let io = interpret("print -1;").unwrap();
    assert_eq!(io.as_slice(), "-1".as_bytes());
}

#[test]
fn calculation_precedence() {
    let io = interpret("print 2 + 2 * 2 - (3 + 3);").unwrap();
    let out = String::from_utf8(io).unwrap();
    assert_eq!(out, "0");
}

#[test]
fn conditionals() {
    let io = interpret(
        r#"
    let input = 11;
    if (input > 10) {
        print 3;
    } else if (input > 5) {
        print 2;
    } else {
        print 1;
    }
    "#,
    )
    .unwrap();
    let out = String::from_utf8(io).unwrap();
    assert_eq!(out, "3");

    let io = interpret(
        r#"
    let input = 6;
    if (input > 10) {
        print 3;
    } else if (input > 5) {
        print 2;
    } else {
        print 1;
    }
    "#,
    )
    .unwrap();
    let out = String::from_utf8(io).unwrap();
    assert_eq!(out, "2");

    let io = interpret(
        r#"
    let input = 3;
    if (input > 10) {
        print 3;
    } else if (input > 5) {
        print 2;
    } else {
        print 1;
    }
    "#,
    )
    .unwrap();
    let out = String::from_utf8(io).unwrap();
    assert_eq!(out, "1");
}

pub fn interpret(source: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let io = Rc::new(RefCell::new(vec![]));
    {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_program()?;
        let mut compiler = Compiler::default();
        let chunk = compiler.compile_program(ast)?;
        let mut vm = Vm::with_io(io.clone());
        vm.interpret(chunk)?;
    }

    Ok(io.clone().borrow().clone())
}
