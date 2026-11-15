#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalError {
    InvalidDimension(&'static str),
    OutOfBounds(&'static str),
    EmptyInput,
    InvariantViolation(&'static str),
}

impl std::fmt::Display for IntervalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
