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
fn test_unit_booth_min_rotation_rs_d61() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let cases = [
        ("baaa", 1usize, "aaab"),
        ("cba", 2usize, "acb"),
        ("banana", 5usize, "abanan"),
        ("aaaa", 0usize, "aaaa"),
        ("abcd", 0usize, "abcd"),
        ("", 0usize, ""),
    ];
    for (s, start, want) in cases {
        assert_eq!(crate::core::least_rotation(s), start, "start mismatch for {s}");
        assert_eq!(crate::core::minimal_rotation(s), want, "rotation mismatch for {s}");
    }
    let info = crate::core::rotation_info("banana");
    assert_eq!(info.start, 5);
    assert_eq!(info.text, "abanan");
    assert!(crate::invariants::ensure(true).is_ok());
}
