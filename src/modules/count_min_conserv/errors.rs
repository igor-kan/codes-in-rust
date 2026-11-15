#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SketchError {
    InvalidDimension(&'static str),
    OutOfBounds(&'static str),
    EmptyInput,
    InvariantViolation(&'static str),
}

impl std::fmt::Display for SketchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
