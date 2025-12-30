use std::io;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            break;
        }

        let mut iter = input.split_whitespace();

        let (a, op, b) = match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(op), Some(b)) => (a, op, b),
            _ => {
                println!("Invalid format. Use: operand operator operand");
                continue;
            }
        };

        let a: i32 = match a.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid first operand");
                continue;
            }
        };

        let b: i32 = match b.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid second operand");
                continue;
            }
        };

        let op: char = op.chars().next().unwrap();

        let result = match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => {
                println!("Unsupported operator");
                continue;
            }
        };

        println!("> {}", result);
    }
}
