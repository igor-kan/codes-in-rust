#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZMatchError {
    InvalidDimension(&'static str),
    OutOfBounds(&'static str),
    EmptyInput,
    InvariantViolation(&'static str),
}

impl std::fmt::Display for ZMatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
