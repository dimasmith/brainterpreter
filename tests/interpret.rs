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
    assert_eq!(io.as_slice(), "-1\n".as_bytes());
}

#[test]
fn calculation_precedence() {
    let io = interpret("print 2 + 2 * 2 - (3 + 3);").unwrap();
    let out = String::from_utf8(io).unwrap();
    assert_eq!(out, "0\n");
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
    assert_eq!(out, "3\n");

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
    assert_eq!(out, "2\n");

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
    assert_eq!(out, "1\n");
}

#[test]
fn while_loop() {
    let source = r#"
    let i = 5;
    while (i > 0) {
        print i;
        i = i - 1;
    }
    print 100;
    "#;
    let io = interpret(source).unwrap();
    let out = String::from_utf8(io).unwrap();

    assert_eq!(out, "5\n4\n3\n2\n1\n100\n");
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
