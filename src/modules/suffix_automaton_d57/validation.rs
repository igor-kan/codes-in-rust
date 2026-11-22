use crate::errors::ModError;

pub fn check_nonempty<T>(s: &[T]) -> Result<(), ModError> {
    if s.is_empty() {
        return Err(ModError::EmptyInput);
    }
    Ok(())
}
