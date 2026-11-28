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
fn test_prop_binary_lifting_lca_rs_d61() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let parent = [0usize, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7];
    let table = crate::core::build_lca(&parent);
    let n = parent.len();
    for a in 0..n {
        assert_eq!(crate::core::lca(&table, a, a), a);
        for b in 0..n {
            let l = crate::core::lca(&table, a, b);
            assert_eq!(l, crate::core::lca(&table, b, a));
            assert!(table.depth[l] <= table.depth[a]);
            assert!(table.depth[l] <= table.depth[b]);
            assert_eq!(crate::core::lca(&table, l, a), l);
            assert_eq!(crate::core::lca(&table, l, b), l);
        }
    }
}
