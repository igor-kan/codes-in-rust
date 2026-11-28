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
fn test_prop_booth_min_rotation_rs_d61() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let samples = ["banana", "cba", "aaaa", "abab", "z", "mississippi", "abcabcabc"];
    for s in samples {
        let chars: Vec<char> = s.chars().collect();
        let best = crate::core::minimal_rotation(s);
        assert_eq!(best.chars().count(), chars.len());
        for r in 0..chars.len() {
            let rot: String = chars[r..]
                .iter()
                .chain(chars[..r].iter())
                .copied()
                .collect();
            assert!(
                rot.as_str() >= best.as_str(),
                "rotation {r} of {s} is smaller than {best}"
            );
        }
        let start = crate::core::least_rotation(s);
        let rotated: String = chars[start..]
            .iter()
            .chain(chars[..start].iter())
            .copied()
            .collect();
        assert_eq!(rotated, best);
        assert_eq!(crate::core::least_rotation(&best), 0);
    }
}
