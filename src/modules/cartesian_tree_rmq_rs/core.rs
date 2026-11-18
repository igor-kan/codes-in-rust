use crate::types::CartTree;

pub fn build_cartesian(arr: &[i64]) -> CartTree {
    let n = arr.len();
    let mut left = vec![None; n];
    let mut right = vec![None; n];
    let mut st: Vec<usize> = Vec::new();
    for i in 0..n {
        let mut last = None;
        while let Some(&top) = st.last() {
            if arr[top] > arr[i] { last = st.pop(); } else { break; }
        }
        if let Some(&top) = st.last() { right[top] = Some(i); }
        if let Some(l) = last { left[i] = Some(l); }
        st.push(i);
    }
    CartTree { root: st.first().copied(), left, right }
}

pub fn query_rmq_idx(ct: &CartTree, l: usize, r: usize) -> Option<usize> {
    let mut cur = ct.root;
    while let Some(u) = cur {
        if u < l { cur = ct.right[u]; }
        else if u > r { cur = ct.left[u]; }
        else { return Some(u); }
    }
    None
}
