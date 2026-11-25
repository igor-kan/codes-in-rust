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
fn test_prop_boyer_moore_horspool_d60() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let h = b"abracadabra-abracadabra";
    let p = b"abra";
    let got = crate::core::bmh_find_all(h, p);
    let exact: Vec<usize> = (0..=h.len()-p.len()).filter(|&i| &h[i..i+p.len()] == p).collect();
    assert_eq!(got, exact);
}
