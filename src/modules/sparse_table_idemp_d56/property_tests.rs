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
fn test_prop_sparse_table_idemp_d56() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let xs = [12, 4, 19, 3, 8, 22, 6];
    let st = crate::core::SparseMinTable::build(&xs);
    for l in 0..xs.len() {
        for r in l..xs.len() {
            assert_eq!(st.query_min(l, r), *xs[l..=r].iter().min().unwrap());
        }
    }
}
