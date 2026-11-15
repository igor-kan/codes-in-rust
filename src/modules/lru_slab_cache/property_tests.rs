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
fn prop_lru_invariant_under_mixed_workload() {
    let mut lru = create_lru::<u64, u64>(16).unwrap();
    for step in 0..300u64 {
        let k = (step * 7 + 3) % 29;
        if step % 3 == 0 {
            let _ = lru.get(&k);
        } else {
            lru.put(k, step);
        }
        assert!(lru.len() <= 16);
        assert_eq!(verify_doubly_linked_ring(&lru), Ok(()));
    }
}
