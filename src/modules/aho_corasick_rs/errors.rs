#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AhoError {
    InvalidInput(&'static str),
    OutOfBounds(&'static str),
    CapacityFull,
    InvariantViolation(&'static str),
}

impl std::fmt::Display for AhoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
