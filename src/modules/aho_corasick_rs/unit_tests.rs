#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use fast_path::count_all_matches;
use interface::compile_patterns;
use invariants::verify_matches_oracle;

#[test]
fn test_aho_classic_ushers() {
    let pats: &[&[u8]] = &[b"he", b"she", b"his", b"hers"];
    let ac = compile_patterns(pats).unwrap();
    let txt = b"ushers";
    assert_eq!(count_all_matches(&ac, txt), 3);
    assert_eq!(verify_matches_oracle(&ac, pats, txt), Ok(()));
}
