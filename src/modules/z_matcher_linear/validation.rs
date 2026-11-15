use crate::errors::ZMatchError;

pub fn validate_search_slices(text: &[u8], pattern: &[u8]) -> Result<(), ZMatchError> {
    if pattern.is_empty() {
        return Err(ZMatchError::EmptyInput);
    }
    if text.len() < pattern.len() {
        return Err(ZMatchError::OutOfBounds("pattern longer than text"));
    }
    Ok(())
}
