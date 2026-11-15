#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::build_maglev;
use invariants::verify_complete_balanced;

#[test]
fn prop_maglev_rejects_composite_table() {
    assert!(build_maglev(&["a", "b"], 100).is_err());
    let ok = build_maglev(&["a", "b", "c"], 101).unwrap();
    assert_eq!(verify_complete_balanced(&ok), Ok(()));
}
