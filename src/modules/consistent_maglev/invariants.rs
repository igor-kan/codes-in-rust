use crate::errors::MaglevError;
use crate::types::MaglevRouter;

pub fn verify_complete_balanced(router: &MaglevRouter) -> Result<(), MaglevError> {
    if router.lookup.len() != router.table_size || router.lookup.iter().any(|&x| x >= router.backends.len()) {
        return Err(MaglevError::InvariantViolation("incomplete or out-of-range lookup table"));
    }
    Ok(())
}
