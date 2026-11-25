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
fn test_prop_rope_weight_split_d60() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let r = crate::core::RopeNode::concat(
        crate::core::RopeNode::from_str("alpha-"),
        crate::core::RopeNode::concat(
            crate::core::RopeNode::from_str("beta-"),
            crate::core::RopeNode::from_str("gamma"),
        ),
    );
    let (l, rr) = r.split(8);
    let merged = crate::core::RopeNode::concat(l, rr);
    let mut out = String::new();
    merged.collect_into(&mut out);
    assert_eq!(out, "alpha-beta-gamma");
}
