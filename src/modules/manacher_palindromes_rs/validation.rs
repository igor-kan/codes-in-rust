use crate::errors::ManacherError;
use crate::types::PalindromeProfile;

pub fn validate_profile(s: &[u8], p: &PalindromeProfile) -> Result<(), ManacherError> {
    if p.d_odd.len() != s.len() || p.d_even.len() != s.len() {
        return Err(ManacherError::InvariantViolation("vector length mismatch"));
    }
    Ok(())
}
