use crate::errors::DstError;
use crate::types::DisjointSparseTable;

pub fn validate_range(t: &DisjointSparseTable, l: usize, r: usize) -> Result<(), DstError> {
    if l > r || r >= t.orig_len {
        return Err(DstError::OutOfBounds("invalid query interval"));
    }
    Ok(())
}
