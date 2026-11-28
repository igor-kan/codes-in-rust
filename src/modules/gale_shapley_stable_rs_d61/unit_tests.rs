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
fn test_unit_gale_shapley_stable_rs_d61() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let pa = vec![vec![0, 1], vec![1, 0]];
    let pb = vec![vec![1, 0], vec![0, 1]];
    let got = crate::core::stable_match(&pa, &pb);
    assert_eq!(got, vec![0, 1]);
    let pa2 = vec![vec![1, 0], vec![0, 1]];
    let pb2 = vec![vec![1, 0], vec![0, 1]];
    assert_eq!(crate::core::stable_match(&pa2, &pb2), vec![1, 0]);
    let pairs = crate::core::matched_pairs(&got);
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0], crate::types::StablePair { a: 0, b: 0 });
    assert!(crate::invariants::ensure(true).is_ok());
}
