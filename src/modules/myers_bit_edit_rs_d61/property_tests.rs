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
fn test_prop_myers_bit_edit_rs_d61() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    fn reference(a: &str, b: &str) -> usize {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut prev: Vec<usize> = (0..=b.len()).collect();
        let mut cur = vec![0usize; b.len() + 1];
        for i in 1..=a.len() {
            cur[0] = i;
            for j in 1..=b.len() {
                let cost = usize::from(a[i - 1] != b[j - 1]);
                cur[j] = (prev[j] + 1)
                    .min(cur[j - 1] + 1)
                    .min(prev[j - 1] + cost);
            }
            std::mem::swap(&mut prev, &mut cur);
        }
        prev[b.len()]
    }
    let pairs = [
        ("kitten", "sitting"),
        ("flaw", "lawn"),
        ("intention", "execution"),
        ("aaaa", "aa"),
        ("abc", ""),
    ];
    for (a, b) in pairs {
        assert_eq!(crate::core::myers_distance(a, b), reference(a, b));
        assert_eq!(
            crate::core::myers_distance(a, b),
            crate::core::myers_distance(b, a)
        );
    }
}
