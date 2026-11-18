use crate::errors::ModError;

pub fn ensure(ok: bool) -> Result<(), ModError> {
    if !ok {
        return Err(ModError::InvariantViolation("postcondition check failed"));
    }
    Ok(())
}
