#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VebError {
    InvalidInput(&'static str),
    OutOfBounds(&'static str),
    CapacityFull,
    InvariantViolation(&'static str),
}

impl std::fmt::Display for VebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
