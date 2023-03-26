use l9_vm::interpret;

#[test]
fn expression_with_negative_numbers() {
    assert!(interpret("5 + 20 - 4").is_ok());
}
