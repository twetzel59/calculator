//! This module provides a representation of
//! mathematical expressions that the calculator
//! can process directly.

/// Represents an expression consisting of
/// one or more operations and their operands.
#[derive(Clone, Debug)]
pub struct Expression {
    components: Vec<Component>,
}

impl Expression {
    /// Create a new instance of ``Expression``
    /// containing no ``Components``.
    pub fn new() -> Expression {
        Expression {
            components: Vec::new(),
        }
    }

    /// Append a ``Component`` to the ``Expression``.
    pub fn push(&mut self, component: Component) {
        self.components.push(component);
    }
}

/// The type used for numbers in the calculator.
pub type Number = i64;

/// Represents an individual scalar *or* operator
/// in an expression. In other words, this type
/// provides a higher-level representation of
/// either a number or operator, such as ``+``.
#[derive(Clone, Copy, Debug)]
pub enum Component {
    /// A whole number.
    Num(Number),

    /// An operator.
    Op(Operator),
}

/// Represents the various operators that the
/// calculator supports.
#[derive(Clone, Copy, Debug)]
pub enum Operator {
    /// Addition: ``+``
    Add,

    /// Multiplication: ``*``
    Multiply,
}
