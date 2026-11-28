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
fn test_unit_douglas_peucker_rs_d61() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let line = [
        crate::types::Pt { x: 0.0, y: 0.0 },
        crate::types::Pt { x: 1.0, y: 0.1 },
        crate::types::Pt { x: 2.0, y: 0.0 },
        crate::types::Pt { x: 3.0, y: 1.0 },
        crate::types::Pt { x: 4.0, y: 0.0 },
    ];
    let simp = crate::core::simplify(&line, 0.7);
    assert_eq!(simp.len(), 3);
    assert_eq!(simp[1].x, 3.0);
    let d = crate::core::perpendicular_distance(
        crate::types::Pt { x: 0.0, y: 2.0 },
        crate::types::Pt { x: 0.0, y: 0.0 },
        crate::types::Pt { x: 0.0, y: 0.0 },
    );
    assert_eq!(d, 2.0);
    assert!(crate::invariants::ensure(true).is_ok());
}
