#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use fast_path::longest_common_substring_len;
use interface::build_sam;
use invariants::verify_suffix_links;
use validation::validate_sam_size;

#[test]
fn test_sam_abcbc_and_lcs() {
    let sam = build_sam(b"abcbc").unwrap();
    assert_eq!(validate_sam_size(&sam), Ok(()));
    assert_eq!(verify_suffix_links(&sam), Ok(()));
    // Distinct non-empty substrings of "abcbc" (len 5):
    // 1: a,b,c (3); 2: ab,bc,cb (3); 3: abc,bcb,cbc (3); 4: abcb,bcbc (2); 5: abcbc (1) => 12
    assert_eq!(sam.distinct_substrings(), 12);
    assert!(sam.contains_substring(b"bcb"));
    assert!(!sam.contains_substring(b"cba"));
    assert_eq!(longest_common_substring_len(&sam, b"zzbcbcqq"), 4);
}
