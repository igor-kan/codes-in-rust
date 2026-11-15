#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

use interface::compile_horspool;
use invariants::verify_bmh_matches;
use validation::validate_shift_table;

#[test]
fn test_bmh_overlapping_note() {
    let tbl = compile_horspool(b"GCAGAGAG").unwrap();
    assert_eq!(validate_shift_table(&tbl), Ok(()));
    let hay = b"GCATCGCAGAGAGTATACAGTACG";
    let res = tbl.find_all(hay);
    assert_eq!(res, vec![5]);
    assert_eq!(verify_bmh_matches(&tbl, hay, &res), Ok(()));
    assert_eq!(tbl.find_first(hay), Some(5));
}
