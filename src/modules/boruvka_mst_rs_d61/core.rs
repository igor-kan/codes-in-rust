use crate::types::{MstSummary, WeightedEdge};

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}

pub fn boruvka_mst(n: usize, edges: &[WeightedEdge]) -> MstSummary {
    if n == 0 {
        return MstSummary { weight: 0, edges: 0 };
    }
    let mut dsu = DisjointSet::new(n);
    let mut total = 0i64;
    let mut used = 0usize;
    loop {
        let mut best: Vec<Option<usize>> = vec![None; n];
        for (idx, e) in edges.iter().enumerate() {
            if e.u >= n || e.v >= n {
                continue;
            }
            let ru = dsu.find(e.u);
            let rv = dsu.find(e.v);
            if ru == rv {
                continue;
            }
            match best[ru] {
                None => best[ru] = Some(idx),
                Some(cur) => {
                    if e.w < edges[cur].w {
                        best[ru] = Some(idx);
                    }
                }
            }
            match best[rv] {
                None => best[rv] = Some(idx),
                Some(cur) => {
                    if e.w < edges[cur].w {
                        best[rv] = Some(idx);
                    }
                }
            }
        }
        let mut merged = false;
        for idx in best.iter().flatten() {
            let e = edges[*idx];
            if dsu.union(e.u, e.v) {
                total += e.w;
                used += 1;
                merged = true;
            }
        }
        if !merged {
            break;
        }
    }
    MstSummary { weight: total, edges: used }
}
