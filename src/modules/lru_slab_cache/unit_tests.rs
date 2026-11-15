#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::create_lru;
use invariants::verify_doubly_linked_ring;

#[test]
fn test_lru_eviction_order() {
    let mut lru = create_lru::<&str, i32>(2).unwrap();
    assert_eq!(lru.put("a", 1), None);
    assert_eq!(lru.put("b", 2), None);
    assert_eq!(lru.get(&"a"), Some(&1)); // "b" is now LRU
    assert_eq!(lru.put("c", 3), Some(("b", 2)));
    assert_eq!(lru.get(&"b"), None);
    assert_eq!(lru.get(&"a"), Some(&1));
    assert_eq!(verify_doubly_linked_ring(&lru), Ok(()));
}
