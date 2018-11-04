//! This module provides a representation of
//! mathematical expressions that the calculator
//! can process directly.

/// Represents an expression consisting of
/// one or more operations and their operands.
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

/// Represents an individual scalar *or* operator
/// in an expression. In other words, this type
/// provides a higher-level representation of
/// either a number or operator, such as ``+``.
pub enum Component {
    /// A whole number.
    Scalar(u32),

    /// An addition operator.
    OpAdd,

    /// A multiplication operator.
    OpMul,
}
