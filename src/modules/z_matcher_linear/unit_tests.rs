#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use core::z_array;
use interface::match_pattern;
use invariants::verify_z_definition;

#[test]
fn test_z_sentinel_free_binary_safe() {
    let text = b"ababa\x00\x01ababa\xffababa";
    let pat = b"aba";
    let rep = match_pattern(text, pat).unwrap();
    assert_eq!(rep.offsets, vec![0, 2, 7, 9, 13, 15]);
    let z = z_array(b"aabxaabxcaabxaabxay");
    assert_eq!(z[9], 8);
    assert_eq!(verify_z_definition(b"aabxaabxcaabxaabxay", &z), Ok(()));
}
