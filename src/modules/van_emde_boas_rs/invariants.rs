use crate::errors::VebError;
use crate::types::VebTree;
use std::collections::BTreeSet;

pub fn verify_veb_oracle(t: &VebTree, oracle: &BTreeSet<usize>) -> Result<(), VebError> {
    for q in 0..t.u {
        if t.contains(q) != oracle.contains(&q) {
            return Err(VebError::InvariantViolation("membership mismatch"));
        }
        let v_succ = t.successor(q);
        let o_succ = oracle.range((q + 1)..).next().copied();
        if v_succ != o_succ {
            return Err(VebError::InvariantViolation("successor mismatch"));
        }
    }
    Ok(())
}
