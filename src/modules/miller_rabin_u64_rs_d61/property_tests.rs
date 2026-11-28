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
fn test_prop_miller_rabin_u64_rs_d61() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    fn trial_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        let mut d = 2u64;
        while d * d <= n {
            if n % d == 0 {
                return false;
            }
            d += 1;
        }
        true
    }
    for n in 0u64..2000 {
        assert_eq!(
            crate::core::is_prime(n),
            trial_prime(n),
            "mismatch at {n}"
        );
    }
    let mersenne = (1u64 << 61) - 1;
    assert!(crate::core::is_prime(mersenne));
    assert!(!crate::core::is_prime(mersenne * 3));
}
