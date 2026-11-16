use crate::errors::VebError;
use crate::types::VebTree;

pub fn validate_key(t: &VebTree, x: usize) -> Result<(), VebError> {
    if x >= t.u {
        return Err(VebError::OutOfBounds("key exceeds universe U"));
    }
    Ok(())
}
