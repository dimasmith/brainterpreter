use l9_vm::interpret;

#[test]
fn expression_with_negative_numbers() {
    assert!(interpret("2 + 4 * 8").is_ok());
}
