use crate::errors::LruError;
use crate::types::LruSlab;
use std::hash::Hash;

pub fn validate_capacity<K: Eq + Hash + Clone, V>(cache: &LruSlab<K, V>) -> Result<(), LruError> {
    if cache.cap == 0 || cache.map.len() > cache.cap {
        return Err(LruError::OutOfBounds("cache exceeds capacity"));
    }
    Ok(())
}
