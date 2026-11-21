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
fn test_unit_sparse_table_idemp_d56() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let st = crate::core::SparseMinTable::build(&[5, 2, 9, 1, 7, 3, 8]);
    assert_eq!(st.query_min(0, 2), 2);
    assert_eq!(st.query_min(2, 6), 1);
    assert_eq!(st.query_min(4, 6), 3);
    assert!(crate::invariants::ensure(true).is_ok());
}
