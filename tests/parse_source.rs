use l9_vm::ast::Program;
use l9_vm::lexer::Lexer;
use l9_vm::parser::{Parser, ParsingError};

#[test]
fn parse_array_element_read() {
    let source = r#"
        i = i + 1;
    "#;
    let _ = parse(source).unwrap();

    // assert_eq!(
    //     program.statements(),
    //     vec![Statement::Expression(Expression::ArrayIndex(
    //         Box::new(Expression::Variable("greet".to_string())),
    //         Box::new(Expression::number(0))
    //     ))]
    // );
}

fn parse(source: &str) -> Result<Program, ParsingError> {
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(&mut lexer);
    parser.parse_program()
}
