use regex::Regex;

pub fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let re = Regex::new(r"\d+|[+\-*/()]")
        .map_err(|_| "Failed to compile regex".to_string())?;

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

pub fn parse_expression(input: &str) -> Result<(i32, char, i32), String> {
    let tokens = tokenize(input)?;

    if tokens.len() != 3 {
        return Err("Expected format: operand operator operand".to_string());
    }

    let a: i32 = tokens[0]
        .parse()
        .map_err(|_| "Invalid first operand".to_string())?;

    let op: char = tokens[1]
        .chars()
        .next()
        .ok_or("Missing operator".to_string())?;

    let b: i32 = tokens[2]
        .parse()
        .map_err(|_| "Invalid second operand".to_string())?;

    Ok((a, op, b))
}

pub fn evaluate(a: i32, op: char, b: i32) -> Result<i32, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err("Unsupported operator".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -------- tokenize tests --------
    #[test]
    fn test_tokenize_simple_expression() {
        let input = "2+3";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec!["2", "+", "3"]);
    }

    #[test]
    fn test_tokenize_with_spaces() {
        let input = "10 * 5";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec!["10", "*", "5"]);
    }

    #[test]
    fn test_tokenize_invalid_input() {
        let input = "abc";
        let result = tokenize(input);
        assert!(result.is_err());
    }

    // -------- parse_expression tests --------
    #[test]
    fn test_parse_expression_valid() {
        let input = "8-3";
        let result = parse_expression(input).unwrap();
        assert_eq!(result, (8, '-', 3));
    }

    #[test]
    fn test_parse_expression_invalid_format() {
        let input = "1+2+3";
        let result = parse_expression(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_expression_invalid_operand() {
        let input = "a+2";
        let result = parse_expression(input);
        assert!(result.is_err());
    }

    // -------- evaluate tests --------
    #[test]
    fn test_evaluate_addition() {
        assert_eq!(evaluate(2, '+', 3).unwrap(), 5);
    }

    #[test]
    fn test_evaluate_division_by_zero() {
        let result = evaluate(10, '/', 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_invalid_operator() {
        let result = evaluate(1, '%', 2);
        assert!(result.is_err());
    }
}
