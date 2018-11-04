//! This module provides for the parsing
//! and representation of special calculator
//! commands.

/// Represents command actions in ready-to-process form.
/// Obtained by parsing a command with ``handle_cmd``.
pub enum Command {
    /// The user wishes to exit the calculator.
    Quit,
}

impl Command {
    /// Handles commands. The colon should not be
    /// included in the parameter.
    /// If the command is valid, returns an instance
    /// of ``Command`` indicating what the command
    /// should do.
    /// If the command is invalid, returns an
    /// an empty ``Err`` (it is of the unit type, ``()``).
    pub fn handle_cmd(cmd: &str) -> Result<Command, ()> {
        if cmd == "q" {
            Ok(Command::Quit)
        } else {
            Err(())
        }
    }
}
