use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Slot<K, V> {
    pub key: K,
    pub val: V,
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct LruSlab<K: Eq + Hash + Clone, V> {
    pub cap: usize,
    pub entries: Vec<Option<Slot<K, V>>>,
    pub free: Vec<usize>,
    pub map: HashMap<K, usize>,
    pub head: Option<usize>,
    pub tail: Option<usize>,
}
