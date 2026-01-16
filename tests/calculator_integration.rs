use calculator::{parse_expression, evaluate};

#[test]
fn test_full_expression_flow() {
    let input = "6*7";

    let (a, op, b) = parse_expression(input).unwrap();
    let result = evaluate(a, op, b).unwrap();

    assert_eq!(result, 42);
}

#[test]
fn test_invalid_expression_flow() {
    let input = "4/0";

    let (a, op, b) = parse_expression(input).unwrap();
    let result = evaluate(a, op, b);

    assert!(result.is_err());
}
