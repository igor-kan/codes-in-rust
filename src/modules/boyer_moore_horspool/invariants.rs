use crate::errors::BmhError;
use crate::types::HorspoolTable;

pub fn verify_bmh_matches(tbl: &HorspoolTable, haystack: &[u8], got: &[usize]) -> Result<(), BmhError> {
    let m = tbl.needle.len();
    for &idx in got {
        if idx + m > haystack.len() || &haystack[idx..idx + m] != tbl.needle.as_slice() {
            return Err(BmhError::InvariantViolation("invalid match index"));
        }
    }
    Ok(())
}
