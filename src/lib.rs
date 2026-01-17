use regex::Regex;
use std::collections::VecDeque;

/* ---------------- TOKENIZATION ---------------- */

pub fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let re = Regex::new(r"\d+|[+\-*/()]")
        .map_err(|_| "Regex compile error".to_string())?;

    let tokens: Vec<String> = re
        .find_iter(input)
        .map(|m| m.as_str().to_string())
        .collect();

    if tokens.is_empty() {
        Err("No valid tokens found".to_string())
    } else {
        Ok(tokens)
    }
}

/* ---------------- OPERATOR HELPERS ---------------- */

fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}

fn is_operator(token: &str) -> bool {
    matches!(token, "+" | "-" | "*" | "/")
}

/* ---------------- SHUNTING YARD (INFIX → RPN) ---------------- */

fn to_rpn(tokens: Vec<String>) -> Result<Vec<String>, String> {
    let mut output = Vec::new();
    let mut operators: VecDeque<String> = VecDeque::new();

    for token in tokens {
        if token.chars().all(|c| c.is_digit(10)) {
            output.push(token);
        } else if is_operator(&token) {
            while let Some(top) = operators.back() {
                if is_operator(top) && precedence(top) >= precedence(&token) {
                    output.push(operators.pop_back().unwrap());
                } else {
                    break;
                }
            }
            operators.push_back(token);
        } else if token == "(" {
            operators.push_back(token);
        } else if token == ")" {
            while let Some(top) = operators.pop_back() {
                if top == "(" {
                    break;
                }
                output.push(top);
            }
        } else {
            return Err(format!("Invalid token: {}", token));
        }
    }

    while let Some(op) = operators.pop_back() {
        if op == "(" {
            return Err("Mismatched parentheses".to_string());
        }
        output.push(op);
    }

    Ok(output)
}

/* ---------------- RPN EVALUATION ---------------- */

fn evaluate_rpn(tokens: Vec<String>) -> Result<i32, String> {
    let mut stack: Vec<i32> = Vec::new();

    for token in tokens {
        if let Ok(num) = token.parse::<i32>() {
            stack.push(num);
        } else if is_operator(&token) {
            let b = stack.pop().ok_or("Invalid expression")?;
            let a = stack.pop().ok_or("Invalid expression")?;

            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0 {
                        return Err("Division by zero".to_string());
                    }
                    a / b
                }
                _ => return Err("Unknown operator".to_string()),
            };

            stack.push(result);
        }
    }

    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err("Invalid expression".to_string())
    }
}

/* ---------------- PUBLIC API ---------------- */

pub fn evaluate_expression(input: &str) -> Result<i32, String> {
    let tokens = tokenize(input)?;
    let rpn = to_rpn(tokens)?;
    evaluate_rpn(rpn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_operations() {
        assert_eq!(evaluate_expression("2+3").unwrap(), 5);
        assert_eq!(evaluate_expression("10-4").unwrap(), 6);
        assert_eq!(evaluate_expression("3*4").unwrap(), 12);
        assert_eq!(evaluate_expression("8/2").unwrap(), 4);
    }

    #[test]
    fn test_multiple_operations() {
        assert_eq!(evaluate_expression("2+3+4").unwrap(), 9);
        assert_eq!(evaluate_expression("10-3-2").unwrap(), 5);
    }

    #[test]
    fn test_bodmas_precedence() {
        assert_eq!(evaluate_expression("2+3*4").unwrap(), 14);
        assert_eq!(evaluate_expression("20-10/2").unwrap(), 15);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(evaluate_expression("(2+3)*4").unwrap(), 20);
        assert_eq!(evaluate_expression("10*(2+3)").unwrap(), 50);
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(evaluate_expression("5*(2+(3*4))").unwrap(), 70);
    }

    #[test]
    fn test_complex_expression() {
        assert_eq!(
            evaluate_expression("5*(2+16)/2-4*(2+2)+1/4").unwrap(),
            29
        );
    }

    #[test]
    fn test_negative_result() {
        assert_eq!(evaluate_expression("30-50").unwrap(), -20);
        assert_eq!(evaluate_expression("(30-70)").unwrap(), -40);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(evaluate_expression("10/0").is_err());
    }

    #[test]
    fn test_invalid_expression() {
        assert!(evaluate_expression("2++3").is_err());
        assert!(evaluate_expression("(2+3").is_err());
        assert!(evaluate_expression(")").is_err());
    }
}