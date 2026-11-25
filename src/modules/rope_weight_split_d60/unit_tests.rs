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
fn test_unit_rope_weight_split_d60() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let r = crate::core::RopeNode::concat(
        crate::core::RopeNode::from_str("hello "),
        crate::core::RopeNode::from_str("rustacean"),
    );
    assert_eq!(r.len(), 15);
    assert_eq!(r.byte_at(6), Some(b'r'));
    let (a, b) = r.split(6);
    assert_eq!(a.len(), 6);
    assert_eq!(b.len(), 9);
    assert!(crate::invariants::ensure(true).is_ok());
}
