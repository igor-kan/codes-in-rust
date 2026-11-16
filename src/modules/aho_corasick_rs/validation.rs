use crate::errors::AhoError;
use crate::types::AhoAutomaton;

pub fn validate_automaton(ac: &AhoAutomaton) -> Result<(), AhoError> {
    if ac.nodes.is_empty() || ac.pat_lens.is_empty() {
        return Err(AhoError::InvariantViolation("uninitialized automaton"));
    }
    Ok(())
}
