use crate::errors::DstError;
use crate::types::DisjointSparseTable;

pub fn build_mod_mul_table(arr: &[u64], modulus: u64) -> Result<DisjointSparseTable, DstError> {
    if arr.is_empty() || modulus < 2 {
        return Err(DstError::InvalidInput("empty input or invalid modulus"));
    }
    Ok(DisjointSparseTable::new_mul_mod(arr, modulus))
}
