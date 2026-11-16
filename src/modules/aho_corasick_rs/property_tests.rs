#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use concurrent::batch_scan;
use interface::compile_patterns;
use invariants::verify_matches_oracle;

#[test]
fn prop_overlapping_dictionary() {
    let pats: &[&[u8]] = &[b"a", b"ab", b"bab", b"bc", b"bca", b"c", b"caa"];
    let ac = compile_patterns(pats).unwrap();
    let docs: &[&[u8]] = &[b"abccab", b"caababaabcaa", b"zzzz"];
    for &d in docs {
        assert_eq!(verify_matches_oracle(&ac, pats, d), Ok(()));
    }
    let cnts = batch_scan(&ac, docs);
    assert_eq!(cnts[2], 0);
}
