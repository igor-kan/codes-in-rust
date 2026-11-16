#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DstError {
    InvalidInput(&'static str),
    OutOfBounds(&'static str),
    CapacityFull,
    InvariantViolation(&'static str),
}

impl std::fmt::Display for DstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
