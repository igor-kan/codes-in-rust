use crate::errors::AhoError;
use crate::types::AhoAutomaton;

pub fn compile_patterns(patterns: &[&[u8]]) -> Result<AhoAutomaton, AhoError> {
    if patterns.is_empty() || patterns.iter().any(|p| p.is_empty()) {
        return Err(AhoError::InvalidInput("empty pattern list or pattern"));
    }
    Ok(AhoAutomaton::build_raw(patterns))
}
