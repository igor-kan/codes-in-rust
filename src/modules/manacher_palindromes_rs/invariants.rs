use crate::errors::ManacherError;
use crate::fast_path::total_palindromic_substrings;
use crate::types::PalindromeProfile;

pub fn verify_oracle(s: &[u8], p: &PalindromeProfile) -> Result<(), ManacherError> {
    let mut total = 0usize;
    let mut best = 0usize;
    for l in 0..s.len() {
        for r in l..s.len() {
            let sub = &s[l..=r];
            let mut rev = sub.to_vec();
            rev.reverse();
            if sub == rev.as_slice() {
                total += 1;
                best = best.max(r - l + 1);
            }
        }
    }
    if total_palindromic_substrings(p) != total || p.longest_len != best {
        return Err(ManacherError::InvariantViolation("Manacher oracle mismatch"));
    }
    Ok(())
}
