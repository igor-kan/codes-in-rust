#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManacherError {
    InvalidInput(&'static str),
    OutOfBounds(&'static str),
    CapacityFull,
    InvariantViolation(&'static str),
}

impl std::fmt::Display for ManacherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
