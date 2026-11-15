use crate::types::LruSlab;
use std::hash::Hash;

pub fn peek_no_promote<'a, K: Eq + Hash + Clone, V>(cache: &'a LruSlab<K, V>, key: &K) -> Option<&'a V> {
    let &idx = cache.map.get(key)?;
    cache.entries[idx].as_ref().map(|s| &s.val)
}
