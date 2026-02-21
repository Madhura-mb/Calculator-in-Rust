use std::io;
use std::io::Write;

use calculator::{evaluate, parse_expression};

fn main() {
    let stdin = io::stdin();

    while write_prompt().is_ok() {
        let mut input = String::new();

        if stdin.read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            break;
        }

        let (a, op, b) = match parse_expression(input) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        match evaluate(a, op, b) {
            Ok(result) => println!("  {}", result),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn write_prompt() -> io::Result<()> {
    print!("> ");
    io::stdout().flush()
}
