use regex::Regex;
use once_cell::sync::Lazy;

static TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\d+|[+\-*%/()]").expect("Invalid regex")
});

/* ---------------- TOKENIZATION ---------------- */

pub fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let tokens: Vec<String> = TOKEN_REGEX
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
        '%' => {
            if b == 0 {
                Err("Cannot modulo by zero".to_string())
            } else {
                Ok(a % b)
            }
        }
        _ => Err("Unsupported operator".to_string()),
    }
}
