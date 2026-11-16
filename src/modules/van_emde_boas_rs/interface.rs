use crate::errors::VebError;
use crate::types::VebTree;

pub fn build_veb(universe: usize) -> Result<VebTree, VebError> {
    if universe < 2 || !universe.is_power_of_two() || universe > 65536 {
        return Err(VebError::InvalidInput("universe must be power of two in [2, 65536]"));
    }
    Ok(VebTree::new_unchecked(universe))
}
