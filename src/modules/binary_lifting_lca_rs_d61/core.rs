use crate::types::LcaTable;

pub fn build_lca(parent: &[usize]) -> LcaTable {
    let n = parent.len();
    if n == 0 {
        return LcaTable {
            depth: Vec::new(),
            up: Vec::new(),
        };
    }
    let root = (0..n).find(|&v| parent[v] == v).unwrap_or(0);
    let mut children = vec![Vec::new(); n];
    for v in 0..n {
        if parent[v] != v && parent[v] < n {
            children[parent[v]].push(v);
        }
    }
    let mut depth = vec![0usize; n];
    let mut stack = vec![root];
    while let Some(u) = stack.pop() {
        for &c in &children[u] {
            depth[c] = depth[u] + 1;
            stack.push(c);
        }
    }
    let log = usize::BITS as usize - n.max(1).leading_zeros() as usize + 1;
    let mut up = vec![vec![0usize; n]; log];
    for v in 0..n {
        up[0][v] = parent[v].min(n - 1);
    }
    for k in 1..log {
        for v in 0..n {
            up[k][v] = up[k - 1][up[k - 1][v]];
        }
    }
    LcaTable { depth, up }
}

pub fn lca(table: &LcaTable, mut a: usize, mut b: usize) -> usize {
    if table.depth[a] < table.depth[b] {
        std::mem::swap(&mut a, &mut b);
    }
    let diff = table.depth[a] - table.depth[b];
    for k in 0..table.up.len() {
        if (diff >> k) & 1 == 1 {
            a = table.up[k][a];
        }
    }
    if a == b {
        return a;
    }
    for k in (0..table.up.len()).rev() {
        if table.up[k][a] != table.up[k][b] {
            a = table.up[k][a];
            b = table.up[k][b];
        }
    }
    table.up[0][a]
}
