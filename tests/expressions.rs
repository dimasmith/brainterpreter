use l9_vm::interpret;

#[test]
fn expression_with_negative_numbers() {
    assert!(interpret("print (2 + 2) * 4;").is_ok());
}
