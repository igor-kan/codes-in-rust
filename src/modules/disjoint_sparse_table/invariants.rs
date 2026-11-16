use crate::errors::DstError;
use crate::types::DisjointSparseTable;

pub fn verify_dst_oracle(t: &DisjointSparseTable, arr: &[u64]) -> Result<(), DstError> {
    for l in 0..arr.len() {
        let mut prod = 1u128;
        for r in l..arr.len() {
            prod = (prod * (arr[r] % t.modulus) as u128) % t.modulus as u128;
            if t.query_inclusive(l, r) != prod as u64 {
                return Err(DstError::InvariantViolation("range product mismatch"));
            }
        }
    }
    Ok(())
}
