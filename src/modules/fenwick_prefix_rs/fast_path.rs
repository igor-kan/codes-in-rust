use crate::types::FenwickTree;

/// Returns smallest 1-based prefix length k in [1..=n] such that prefix_sum(k) >= target, or None.
pub fn lower_bound_prefix(bit: &FenwickTree, target: i64) -> Option<usize> {
    let n = bit.raw.len();
    if target <= 0 {
        return if n > 0 { Some(1) } else { None };
    }
    let mut pos = 0usize;
    let mut acc = 0i64;
    let mut step = n.next_power_of_two();
    while step > 0 {
        let nxt = pos + step;
        if nxt <= n && acc + bit.tree[nxt] < target {
            pos = nxt;
            acc += bit.tree[nxt];
        }
        step >>= 1;
    }
    if pos + 1 <= n { Some(pos + 1) } else { None }
}
