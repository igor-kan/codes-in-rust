#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use fast_path::backend_slot_counts;
use interface::build_maglev;
use invariants::verify_complete_balanced;

#[test]
fn test_maglev_balance_and_disruption() {
    let full = build_maglev(&["n0", "n1", "n2", "n3"], 65537).unwrap();
    assert_eq!(verify_complete_balanced(&full), Ok(()));
    let counts = backend_slot_counts(&full);
    assert!(counts.iter().all(|&c| c > 16000 && c < 16800));
    let reduced = build_maglev(&["n0", "n2", "n3"], 65537).unwrap();
    let mut kept = 0usize;
    let total = 2000u64;
    for k in 0..total {
        let b1 = &full.backends[full.route_key(k)];
        if b1 != "n1" {
            let b2 = &reduced.backends[reduced.route_key(k)];
            if b1 == b2 { kept += 1; }
        }
    }
    assert!(kept > 1200, "unexpectedly low key stability: {}", kept);
}
