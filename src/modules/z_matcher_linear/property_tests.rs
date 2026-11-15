#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use core::find_all_z;

#[test]
fn prop_agrees_with_naive_windows() {
    let text = b"mississippi_mississippi_ssi";
    for pat in [&b"ssi"[..], &b"i"[..], &b"mississippi"[..]] {
        let got = find_all_z(text, pat);
        let exp: Vec<usize> = text.windows(pat.len()).enumerate().filter_map(|(i, w)| (w == pat).then_some(i)).collect();
        assert_eq!(got, exp);
    }
}
