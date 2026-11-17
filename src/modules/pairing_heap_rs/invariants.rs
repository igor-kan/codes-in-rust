use crate::core::PairingHeap;
use crate::errors::PairingHeapError;

pub fn verify_heap_order(h: &PairingHeap) -> Result<(), PairingHeapError> {
    for (i, nd) in h.nodes.iter().enumerate() {
        if !nd.active { continue; }
        let mut cur = nd.child;
        while let Some(c) = cur {
            if h.nodes[c].key < nd.key {
                return Err(PairingHeapError::InvariantViolation("parent > child"));
            }
            cur = h.nodes[c].sibling;
        }
        let _ = i;
    }
    Ok(())
}
