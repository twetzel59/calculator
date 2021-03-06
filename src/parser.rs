//! This module provides the mechanism
//! for parsing user input.

use expression::{Component, Expression, Number, Operator};

// Check that the supplied user input
// is correctly formatted.
fn check(input: &str) -> Result<(), &str> {
    // Non-ASCII characters aren't handled.
    if !input.is_ascii() {
        return Err("Found a non-ASCII character");
    }

    // Check that it's only numbers and operators.
    // Other characters aren't allowed.
    if input.contains(|ch: char| !ch.is_ascii_digit() && !ch.is_ascii_whitespace() && !is_op(ch)) {
        return Err("Found an unexpected character");
    }

    // Check that there are no numbers without
    // operators between them.
    for i in input.split(is_op)
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

/// Parses user input into a calculation-ready
/// format.
/// If the input is invalid, returns an ``Err``
/// containing a human-readable explaination
/// of why the input was malformed.
pub fn parse(input: &str) -> Result<Expression, &str> {
    // First, make sure the input is valid.
    // If it is invalid, the string shall not
    // be parsed.
    check(&input)?;

    let nums = input
        .split(is_op)                   // split on operators
        .map(|substr| substr.trim()     // take out whitespace
        .parse::<Number>()              // parse the number
        .unwrap());                     // check() catches invalid numbers.

    let mut ops = input
        .split(|c: char| c.is_ascii_digit()) // split on *digits*!
        .map(|substr| substr.trim())         // remove the whitespace
        .filter(|substr| !substr.is_empty()) // ignore empty strings
        .map(|substr| to_op(substr.chars()   // The substr should contain only one char.
                                  .nth(0)    // That first character is the operator.
                                  .unwrap()) // unwrap the first character
                      .unwrap());            // unwrap the parsed Operator

    let mut expr = Expression::new();
    for n in nums {
        expr.push(Component::Num(n));

        if let Some(op) = ops.next() {
            expr.push(Component::Op(op))
        }
    }

    Ok(expr)
}

// Converts the ``char`` ``character`` into
// an operator, if possible.
// If ``character`` is the character representation
// of an operator, returns ``Ok(Operator)``.
// If it is not possible to perform the conversion,
// returns ``Err(())``.
fn to_op(character: char) -> Result<Operator, ()> {
    match character {
        '+' => Ok(Operator::Add),
        '*' => Ok(Operator::Multiply),
        _ => Err(()),
    }
}

// Returns ``true`` if and only if ``character``
// is an operator.
fn is_op(character: char) -> bool {
    to_op(character).is_ok()
}
