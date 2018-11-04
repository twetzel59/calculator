//! This module provides the mechanism
//! for parsing user input.

/// Check that the supplied user input
/// is correctly formatted.
pub fn check(input: &str) -> Result<(), &str> {
    // Non-ASCII characters aren't handled.
    if !input.is_ascii() {
        return Err("Found a non-ASCII character");
    }

    // Check that it's only numbers and operators.
    // Other characters aren't allowed.
    if input.contains(|ch: char| !ch.is_ascii_digit() && !ch.is_ascii_whitespace()
                                  && ch != '+' && ch != '*') {
        return Err("Found an unexpected character");
    }

    // Check that there are no numbers without
    // operators between them.
    for i in input.split(|ch| ch == '+' || ch == '*')
                  .map(|substr| substr.trim()) {
        if i.contains(char::is_whitespace) {
            return Err("Found whitespace between digits");
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
                return Err("Stranded infix operator before left operand");
            }
        } else if pieces.peek().is_none() {
            // Similarly, the last string from split
            // should be empty for infix operators.
            if !p.is_empty() {
                return Err("Stranded infix operator after right operand");
            }
        } else {
            // There should only be one operator
            // per string. Otherwise, there were
            // multiple operators in a row without
            // numbers between them.
            if p.len() > 1 {
                return Err("Misplaced infix operator");
            }
        }
    }

    // If we're here, the input is okay.
    Ok(())
}
