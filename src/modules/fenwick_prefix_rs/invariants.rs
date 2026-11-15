use crate::errors::FenwickError;
use crate::types::FenwickTree;

pub fn verify_all_prefixes(bit: &FenwickTree) -> Result<(), FenwickError> {
    let mut running = 0i64;
    for (idx, &v) in bit.raw.iter().enumerate() {
        running += v;
        if bit.prefix_sum(idx + 1) != running {
            return Err(FenwickError::InvariantViolation("fenwick prefix diverges from exact sum"));
        }
    }
    Ok(())
}
