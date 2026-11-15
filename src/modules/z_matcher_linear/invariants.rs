use crate::errors::ZMatchError;

pub fn verify_z_definition(s: &[u8], z: &[usize]) -> Result<(), ZMatchError> {
    if z.len() != s.len() {
        return Err(ZMatchError::InvariantViolation("length mismatch"));
    }
    for i in 1..s.len() {
        let zi = z[i];
        if i + zi > s.len() || s[..zi] != s[i..i + zi] {
            return Err(ZMatchError::InvariantViolation("prefix mismatch"));
        }
        if i + zi < s.len() && s[zi] == s[i + zi] {
            return Err(ZMatchError::InvariantViolation("z-value not maximal"));
        }
    }
    Ok(())
}
