pub fn quickselect_kth(xs: &mut [i64], k: usize) -> i64 {
    let mut l = 0usize;
    let mut r = xs.len() - 1;
    while l < r {
        let mid = l + (r - l) / 2;
        xs.swap(mid, r);
        let pivot = xs[r];
        let mut store = l;
        for i in l..r {
            if xs[i] < pivot {
                xs.swap(i, store);
                store += 1;
            }
        }
        xs.swap(store, r);
        if store == k { return xs[store]; }
        if store < k { l = store + 1; } else { r = store - 1; }
    }
    xs[l]
}
