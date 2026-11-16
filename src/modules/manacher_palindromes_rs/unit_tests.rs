#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::analyze_palindromes;
use invariants::verify_oracle;

#[test]
fn test_manacher_known_palindromes() {
    for s in [b"abacaba".as_slice(), b"cbbd".as_slice(), b"aaaa".as_slice()] {
        let p = analyze_palindromes(s).unwrap();
        assert_eq!(verify_oracle(s, &p), Ok(()));
    }
}
