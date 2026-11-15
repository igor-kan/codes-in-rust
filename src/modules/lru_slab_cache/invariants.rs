use crate::errors::LruError;
use crate::types::LruSlab;
use std::hash::Hash;

pub fn verify_doubly_linked_ring<K: Eq + Hash + Clone, V>(cache: &LruSlab<K, V>) -> Result<(), LruError> {
    let mut fwd = Vec::new();
    let mut cur = cache.head;
    while let Some(i) = cur {
        fwd.push(i);
        cur = cache.entries[i].as_ref().ok_or(LruError::InvariantViolation("dangling head chain"))?.next;
        if fwd.len() > cache.cap {
            return Err(LruError::InvariantViolation("cycle detected in forward chain"));
        }
    }
    let mut bwd = Vec::new();
    let mut cur_b = cache.tail;
    while let Some(i) = cur_b {
        bwd.push(i);
        cur_b = cache.entries[i].as_ref().ok_or(LruError::InvariantViolation("dangling tail chain"))?.prev;
        if bwd.len() > cache.cap {
            return Err(LruError::InvariantViolation("cycle detected in backward chain"));
        }
    }
    bwd.reverse();
    if fwd != bwd || fwd.len() != cache.map.len() {
        return Err(LruError::InvariantViolation("forward/backward order or map count mismatch"));
    }
    Ok(())
}
