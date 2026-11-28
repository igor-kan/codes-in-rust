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
fn test_unit_fenwick_2d_rect_rs_d61() {
    assert!(crate::validation::check_nonempty(&[1]).is_ok());
    let mut f = crate::core::Fenwick2D::new(4, 4);
    f.add(1, 1, 5);
    f.add(3, 3, 7);
    f.add(1, 3, -2);
    assert_eq!(
        f.rect_sum(crate::types::RectQuery { x1: 0, y1: 0, x2: 3, y2: 3 }),
        10
    );
    assert_eq!(
        f.rect_sum(crate::types::RectQuery { x1: 1, y1: 1, x2: 1, y2: 1 }),
        5
    );
    assert_eq!(
        f.rect_sum(crate::types::RectQuery { x1: 1, y1: 2, x2: 3, y2: 3 }),
        5
    );
    assert!(crate::invariants::ensure(true).is_ok());
}
