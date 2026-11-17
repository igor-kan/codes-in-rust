use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModError {
    InvalidInput(String),
    OutOfBounds(usize, usize),
    EmptyInput,
    InvariantViolation(&'static str),
}

impl fmt::Display for ModError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            Self::OutOfBounds(i, n) => write!(f, "index {i} out of bounds for len {n}"),
            Self::EmptyInput => write!(f, "empty input"),
            Self::InvariantViolation(msg) => write!(f, "invariant violated: {msg}"),
        }
    }
}

impl std::error::Error for ModError {}
