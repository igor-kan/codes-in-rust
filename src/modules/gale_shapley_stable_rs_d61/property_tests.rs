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
fn test_prop_gale_shapley_stable_rs_d61() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let pa = vec![
        vec![0, 1, 2, 3],
        vec![1, 0, 2, 3],
        vec![2, 1, 0, 3],
        vec![0, 2, 1, 3],
    ];
    let pb = vec![
        vec![3, 1, 2, 0],
        vec![1, 3, 0, 2],
        vec![2, 0, 3, 1],
        vec![0, 1, 2, 3],
    ];
    let m = crate::core::stable_match(&pa, &pb);
    let mut match_b = vec![usize::MAX; 4];
    for (a, &b) in m.iter().enumerate() {
        assert!(b < 4);
        assert_eq!(match_b[b], usize::MAX);
        match_b[b] = a;
    }
    let mut rank_b = vec![vec![0usize; 4]; 4];
    for (b, list) in pb.iter().enumerate() {
        for (r, &a) in list.iter().enumerate() {
            rank_b[b][a] = r;
        }
    }
    for (a, list) in pa.iter().enumerate() {
        for &b in list {
            if b == m[a] {
                break;
            }
            assert!(
                rank_b[b][a] >= rank_b[b][match_b[b]],
                "blocking pair ({a},{b}) in {m:?}"
            );
        }
    }
}
