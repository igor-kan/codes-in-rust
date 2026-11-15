use crate::errors::SamError;
use crate::types::SuffixAutomaton;

pub fn validate_sam_size(sam: &SuffixAutomaton) -> Result<(), SamError> {
    let max_states = if sam.input_len <= 1 { 2 } else { 2 * sam.input_len - 1 };
    if sam.states.len() > max_states {
        return Err(SamError::InvariantViolation("SAM exceeded Blumer 2n-1 state bound"));
    }
    Ok(())
}
