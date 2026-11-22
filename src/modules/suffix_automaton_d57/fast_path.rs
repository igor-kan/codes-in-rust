pub fn fast_acc(xs: &[i64]) -> i64 {
    let mut total: i64 = 0;
    for &x in xs {
        total = total.wrapping_add(x);
    }
    total
}
