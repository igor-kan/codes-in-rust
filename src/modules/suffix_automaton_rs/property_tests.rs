#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::build_sam;
use invariants::verify_suffix_links;
use std::collections::HashSet;

#[test]
fn prop_distinct_substrings_match_hashset() {
    let s = b"banana_ban";
    let sam = build_sam(s).unwrap();
    assert_eq!(verify_suffix_links(&sam), Ok(()));
    let mut set = HashSet::new();
    for i in 0..s.len() {
        for j in (i + 1)..=s.len() {
            set.insert(&s[i..j]);
        }
    }
    assert_eq!(sam.distinct_substrings(), set.len() as u64);
}
