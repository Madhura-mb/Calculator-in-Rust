use std::io;
use std::io::Write;

use calculator::evaluate_expression;

fn main() {
    let stdin = io::stdin();

    while let Ok(_) = write_prompt() {
        let mut input = String::new();

        if stdin.read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            break;
        }

        match evaluate_expression(input) {
            Ok(result) => println!("  {}", result),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn write_prompt() -> io::Result<()> {
    print!("> ");
    io::stdout().flush()
}
