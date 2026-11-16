#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use concurrent::batch_longest_lengths;
use interface::analyze_palindromes;
use invariants::verify_oracle;

#[test]
fn prop_manacher_random_oracle() {
    let mut rng = 0x9988776655443322u64;
    let mut strs = Vec::new();
    for _ in 0..10 {
        let mut buf = Vec::with_capacity(40);
        for _ in 0..40 {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            buf.push(b'a' + ((rng >> 32) % 3) as u8);
        }
        let p = analyze_palindromes(&buf).unwrap();
        assert_eq!(verify_oracle(&buf, &p), Ok(()));
        strs.push(buf);
    }
    let refs: Vec<&[u8]> = strs.iter().map(|v| v.as_slice()).collect();
    let lens = batch_longest_lengths(&refs);
    assert!(lens.iter().all(|&l| l >= 1));
}
