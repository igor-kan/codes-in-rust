#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test]
fn test_pairing_decrease_key() {
    use crate::interface::IndexedMinHeap;
    let mut h = crate::core::PairingHeap::new();
    let id1 = h.push(50, 1);
    let _id2 = h.push(30, 2);
    let id3 = h.push(40, 3);
    h.decrease_key(id1, 10).unwrap();
    crate::invariants::verify_heap_order(&h).unwrap();
    assert_eq!(h.pop_min(), Some((10, 1)));
    h.decrease_key(id3, 25).unwrap();
    assert_eq!(h.pop_min(), Some((25, 3)));
    assert_eq!(h.pop_min(), Some((30, 2)));
}
