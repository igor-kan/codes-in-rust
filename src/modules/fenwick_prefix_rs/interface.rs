use crate::errors::FenwickError;
use crate::types::FenwickTree;

pub fn from_slice_linear(vals: &[i64]) -> FenwickTree {
    let n = vals.len();
    let mut tree = vec![0i64; n + 1];
    for i in 1..=n {
        tree[i] += vals[i - 1];
        let j = i + (i & i.wrapping_neg());
        if j <= n {
            let add = tree[i];
            tree[j] += add;
        }
    }
    FenwickTree { tree, raw: vals.to_vec() }
}

pub fn check_index(n: usize, idx: usize) -> Result<(), FenwickError> {
    if idx >= n {
        return Err(FenwickError::OutOfBounds("index >= n"));
    }
    Ok(())
}
