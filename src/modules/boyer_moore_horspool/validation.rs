use crate::errors::BmhError;
use crate::types::HorspoolTable;

pub fn validate_shift_table(tbl: &HorspoolTable) -> Result<(), BmhError> {
    let m = tbl.needle.len();
    if m == 0 || tbl.shift.iter().any(|&s| s == 0 || s > m) {
        return Err(BmhError::InvariantViolation("shift entry outside [1..=m]"));
    }
    Ok(())
}
