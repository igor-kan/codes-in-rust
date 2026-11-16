use crate::errors::ManacherError;
use crate::types::PalindromeProfile;

pub fn analyze_palindromes(s: &[u8]) -> Result<PalindromeProfile, ManacherError> {
    if s.is_empty() {
        return Err(ManacherError::InvalidInput("empty slice"));
    }
    Ok(crate::core::compute_manacher(s))
}
