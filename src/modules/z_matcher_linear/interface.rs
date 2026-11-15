use crate::errors::ZMatchError;
use crate::types::MatchReport;

pub fn match_pattern(text: &[u8], pattern: &[u8]) -> Result<MatchReport, ZMatchError> {
    if pattern.is_empty() {
        return Err(ZMatchError::EmptyInput);
    }
    let offsets = crate::core::find_all_z(text, pattern);
    let longest_match = if offsets.is_empty() { 0 } else { pattern.len() };
    Ok(MatchReport { offsets, longest_match, overlaps_allowed: true })
}
