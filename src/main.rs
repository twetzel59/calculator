extern crate calculator;

#[macro_use]
extern crate skittles;

use std::io::{self, Write};
use calculator::{Command, parser};

fn main() {
    println!("Command-Line Calculator\n\
              _______________________\n\n\
              Enter an expression, and I'll evaluate it. Type :q to quit.\n");

    //let string = " 3 +5 *  4  ";
    let mut string = String::new();

    loop {
        print!("{}> ", blue!("EVAL"));
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut string).unwrap();

        // If the input was a command starting with
        // a colon, process the command.
        // Otherwise, evaluate the expression as math.
        if string.starts_with(':') {
            if let Ok(cmd) = Command::handle_cmd(string[1..].trim()) {
                match cmd {
                    Command::Quit => return,
                }
            } else {
                error("Invalid command")
            }
        } else {
            match parser::parse(&string) {
                Ok(_) => {},
                Err(msg) => error(msg),
            }
        }

        string.clear();
    }
}

/// Prints the supllied error message with
/// a bold red ``ERR` prefix.
fn error(msg: &str) {
    eprintln!("{}: {}", bold!(red!("ERR")), msg)
}
