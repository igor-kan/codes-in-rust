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
fn test_prop_douglas_peucker_rs_d61() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let mut base = Vec::new();
    for i in 0..50 {
        base.push(crate::types::Pt {
            x: i as f64,
            y: 2.0 * i as f64,
        });
    }
    assert_eq!(crate::core::simplify(&base, 0.001).len(), 2);
    let mut curve = Vec::new();
    for i in 0..64 {
        curve.push(crate::types::Pt {
            x: i as f64,
            y: ((i * i) % 17) as f64 / 4.0,
        });
    }
    let out = crate::core::simplify(&curve, 0.25);
    assert!(out.len() >= 2);
    assert_eq!(out[0], curve[0]);
    assert_eq!(out[out.len() - 1], curve[curve.len() - 1]);
    assert!(out.len() <= curve.len());
    let again = crate::core::simplify(&out, 0.25);
    assert!(again.len() <= out.len());
}
