use crate::errors::LruError;
use crate::types::LruSlab;
use std::collections::HashMap;
use std::hash::Hash;

pub fn create_lru<K: Eq + Hash + Clone, V>(cap: usize) -> Result<LruSlab<K, V>, LruError> {
    if cap == 0 {
        return Err(LruError::InvalidDimension("capacity must be > 0"));
    }
    Ok(LruSlab {
        cap,
        entries: Vec::with_capacity(cap),
        free: Vec::new(),
        map: HashMap::with_capacity(cap),
        head: None,
        tail: None,
    })
}
