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
fn test_unit_binary_lifting_lca_rs_d61() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let chain = crate::core::build_lca(&[0, 0, 1, 2, 3]);
    assert_eq!(chain.node_count(), 5);
    assert_eq!(crate::core::lca(&chain, 4, 3), 3);
    assert_eq!(crate::core::lca(&chain, 4, 1), 1);
    assert_eq!(crate::core::lca(&chain, 2, 2), 2);
    let tree = crate::core::build_lca(&[0, 0, 0, 1, 1, 2, 2]);
    assert_eq!(crate::core::lca(&tree, 3, 4), 1);
    assert_eq!(crate::core::lca(&tree, 3, 5), 0);
    assert_eq!(crate::core::lca(&tree, 5, 6), 2);
    assert!(crate::invariants::ensure(true).is_ok());
}
