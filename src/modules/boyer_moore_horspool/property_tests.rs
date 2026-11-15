#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::compile_horspool;
use invariants::verify_bmh_matches;

#[test]
fn prop_bmh_non_overlapping_window_agreement() {
    let hay = b"xyz_pattern_0123_pattern_456_pattern";
    let pat = b"pattern";
    let tbl = compile_horspool(pat).unwrap();
    let got = tbl.find_all(hay);
    let exp: Vec<usize> = hay.windows(pat.len()).enumerate().filter_map(|(i, w)| (w == pat).then_some(i)).collect();
    assert_eq!(got, exp);
    assert_eq!(verify_bmh_matches(&tbl, hay, &got), Ok(()));
}
