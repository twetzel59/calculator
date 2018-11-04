extern crate calculator;

#[macro_use]
extern crate skittles;

use std::io::{self, Write};

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
            check(&string);
        }

        string.clear();
    }
}

fn check(input: &str) -> bool {
    // Non-ASCII characters aren't handled.
    if !input.is_ascii() {
        error("Found a non-ASCII character");
        return false;
    }

    // Check that its only numbers and operators.
    // Other characters aren't allowed.
    if input.contains(|ch: char| !ch.is_ascii_digit() && !ch.is_ascii_whitespace()
                                  && ch != '+' && ch != '*') {
        error("Found an unexpected character");
        return false;
    }

    // Check that there are no numbers without
    // operators between them.
    for i in input.split(|ch| ch == '+' || ch == '*')
                  .map(|substr| substr.trim()) {
        if i.contains(char::is_whitespace) {
            error("Found whitespace between digits");
            return false;
        }
    }

    // Make sure the operators are not present
    // without numbers on both sides.
    let mut pieces = input.split(|c: char| c.is_ascii_digit())
                          .map(|substr| substr.trim())
                          .enumerate()
                          .peekable();
    while let Some((idx, p)) = pieces.next() {
        if idx == 0 {
            // Since these are infix operators,
            // the first string from split should
            // be empty if the input is correct.
            if !p.is_empty() {
                error("Stranded infix operator before left operand");
                return false;
            }
        } else if pieces.peek().is_none() {
            // Similarly, the last string from split
            // should be empty for infix operators.
            if !p.is_empty() {
                error("Stranded infix operator after right operand");
                return false;
            }
        } else {
            // There should only be one operator
            // per string. Otherwise, there were
            // multiple operators in a row without
            // numbers between them.
            if p.len() > 1 {
                error("Misplaced infix operator");
                return false;
            }
        }
    }

    // If we're here, the input is okay.
    true
}

/// Prints the supllied error message with
/// a bold red ``ERR` prefix.
fn error(msg: &str) {
    eprintln!("{}: {}", bold!(red!("ERR")), msg)
}

/// Represents command actions in ready-to-process
/// form. Obtained by parsing a command with ``handle_cmd``.
enum Command {
    Quit,
}

impl Command {
    /// Handles commands. The colon should not be
    /// included in the parameter.
    /// If the command is valid, returns an instance
    /// of ``Command`` indicating what the command
    /// should do.
    /// If the command is invalid, returns an
    /// an empty ``Err`` (it contains the unit type,
    /// `()`).
    fn handle_cmd(cmd: &str) -> Result<Command, ()> {
        if cmd == "q" {
            Ok(Command::Quit)
        } else {
            Err(())
        }
    }
}

/*
    Below are old attempts at parsing, saved for reference.
*/

/*
fn carrot(fragment: &str, pos: usize) {
    println!("{}", fragment);

    for i in 0..pos {
        print!(" ")
    }

    println!("^");
}
*/

/*
let v = string.split_whitespace()
              .map(|substr| substr.split(|ch| ch == '+' || ch == '*')
                                  .collect::<String>())
              .filter(|s| !s.is_empty())
              .collect::<Vec<_>>();


*/

/*
assert!(string.is_ascii());

for (idx, substr) in string.split_whitespace().enumerate() {
    println!("{:?}", substr);

    if idx % 2 == 0 {
        assert!(substr.find(|ch: char| !ch.is_ascii_digit()).is_none())
    } else {
        assert!(substr.find(|ch: char| !ch.is_ascii_digit() && ch != '+' && ch != '*'))
    }
}
*/
