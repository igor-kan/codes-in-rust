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
fn test_prop_fenwick_2d_rect_rs_d61() {
    assert_eq!(crate::concurrent::parallel_acc(&[1, 2], &[3, 4]), 10);
    let mut f = crate::core::Fenwick2D::new(5, 6);
    let mut grid = vec![vec![0i64; 6]; 5];
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            let v = ((i * 7 + j * 3) % 11) as i64;
            *cell = v;
            f.add(i, j, v);
        }
    }
    for x1 in 0..5 {
        for y1 in 0..6 {
            for x2 in x1..5 {
                for y2 in y1..6 {
                    let mut want = 0i64;
                    for row in grid.iter().take(x2 + 1).skip(x1) {
                        for &v in row.iter().take(y2 + 1).skip(y1) {
                            want += v;
                        }
                    }
                    assert_eq!(
                        f.rect_sum(crate::types::RectQuery { x1, y1, x2, y2 }),
                        want
                    );
                }
            }
        }
    }
}
