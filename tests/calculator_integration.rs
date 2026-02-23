use calculator::evaluate_expression;

#[test]
fn integration_basic_expressions() {
    assert_eq!(evaluate_expression("1+2").unwrap(), 3);
    assert_eq!(evaluate_expression("6/3").unwrap(), 2);
}

#[test]
fn integration_bodmas_and_parentheses() {
    assert_eq!(evaluate_expression("2+3*4").unwrap(), 14);
    assert_eq!(evaluate_expression("(2+3)*4").unwrap(), 20);
}

#[test]
fn integration_large_expression() {
    let expr = "100 - 20 * 3 + 10 + (30 -70)";
    assert_eq!(evaluate_expression(expr).unwrap(), 10);
}

#[test]
fn integration_invalid_inputs() {
    assert!(evaluate_expression("").is_err());
    assert!(evaluate_expression("abc").is_err());
    assert!(evaluate_expression("10/(5-5)").is_err());
}
