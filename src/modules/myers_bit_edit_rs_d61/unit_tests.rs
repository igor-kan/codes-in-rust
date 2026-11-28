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
fn test_unit_myers_bit_edit_rs_d61() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    assert_eq!(crate::core::myers_distance("kitten", "sitting"), 3);
    assert_eq!(crate::core::myers_distance("", "abc"), 3);
    assert_eq!(crate::core::myers_distance("abc", ""), 3);
    assert_eq!(crate::core::myers_distance("abc", "abc"), 0);
    assert_eq!(crate::core::myers_distance("ab", "ba"), 2);
    let long = "0123456789012345678901234567890123456789012345678901234567890123456789";
    assert_eq!(crate::core::myers_distance(long, long), 0);
    let stats = crate::core::edit_stats("clinic", "clinic");
    assert_eq!(stats.distance, 0);
    assert_eq!(stats.pattern_len, 6);
    assert!(crate::invariants::ensure(true).is_ok());
}
