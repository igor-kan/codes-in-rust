use crate::errors::PairingHeapError;
use crate::types::Node;

pub fn validate_handle(nodes: &[Node], id: usize) -> Result<(), PairingHeapError> {
    if id >= nodes.len() {
        return Err(PairingHeapError::OutOfBounds(id, nodes.len()));
    }
    if !nodes[id].active {
        return Err(PairingHeapError::InvalidInput("inactive node".into()));
    }
    Ok(())
}
