use crate::errors::SwissMapError;
use crate::types::SwissFlatMap;

pub trait FlatDict {
    fn insert(&mut self, key: u64, val: i64) -> Option<i64>;
    fn get(&self, key: u64) -> Option<i64>;
    fn remove(&mut self, key: u64) -> Option<i64>;
}

pub fn with_capacity(cap_hint: usize, seed: u64) -> Result<SwissFlatMap, SwissMapError> {
    if cap_hint == 0 {
        return Err(SwissMapError::InvalidInput("capacity must be > 0"));
    }
    let cap = cap_hint.next_power_of_two().max(16);
    Ok(SwissFlatMap {
        ctrl: vec![crate::types::CTRL_EMPTY; cap],
        slots: vec![crate::types::FlatSlot { key: 0, val: 0 }; cap],
        len: 0,
        tombstones: 0,
        seed,
    })
}
