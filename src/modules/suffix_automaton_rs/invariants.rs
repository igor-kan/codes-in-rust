use crate::errors::SamError;
use crate::types::SuffixAutomaton;

pub fn verify_suffix_links(sam: &SuffixAutomaton) -> Result<(), SamError> {
    for (i, st) in sam.states.iter().enumerate().skip(1) {
        let lk = st.link.ok_or(SamError::InvariantViolation("non-root state missing suffix link"))?;
        if sam.states[lk].len >= st.len {
            return Err(SamError::InvariantViolation("suffix link len >= state len"));
        }
        let _ = i;
    }
    Ok(())
}
