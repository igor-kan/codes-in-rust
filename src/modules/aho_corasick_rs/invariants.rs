use crate::errors::AhoError;
use crate::types::AhoAutomaton;
use std::collections::BTreeSet;

pub fn verify_matches_oracle(ac: &AhoAutomaton, patterns: &[&[u8]], text: &[u8]) -> Result<(), AhoError> {
    let got: BTreeSet<(usize, usize)> = ac.find_all(text).into_iter().collect();
    let mut expected = BTreeSet::new();
    for (pid, pat) in patterns.iter().enumerate() {
        if pat.len() <= text.len() {
            for pos in 0..=(text.len() - pat.len()) {
                if &text[pos..(pos + pat.len())] == *pat {
                    expected.insert((pos, pid));
                }
            }
        }
    }
    if got != expected {
        return Err(AhoError::InvariantViolation("Aho-Corasick matches diverge from oracle"));
    }
    Ok(())
}
