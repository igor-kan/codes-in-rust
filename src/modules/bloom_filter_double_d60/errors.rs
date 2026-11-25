use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModError {
    InvalidInput(&'static str),
    EmptyInput,
    InvariantViolation(&'static str),
}

impl fmt::Display for ModError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(m) => write!(f, "invalid input: {m}"),
            Self::EmptyInput => write!(f, "empty input"),
            Self::InvariantViolation(m) => write!(f, "invariant violated: {m}"),
        }
    }
}

impl std::error::Error for ModError {}
