pub struct ChunkedIndex {
    blocks: Vec<Vec<(i64, i64)>>,
    cap: usize,
}

impl ChunkedIndex {
    pub fn new(cap: usize) -> Self { Self { blocks: vec![Vec::new()], cap: cap.max(2) } }
    pub fn insert(&mut self, key: i64, val: i64) {
        let mut bi = 0usize;
        while bi + 1 < self.blocks.len() && self.blocks[bi + 1][0].0 <= key {
            bi += 1;
        }
        let blk = &mut self.blocks[bi];
        match blk.binary_search_by_key(&key, |x| x.0) {
            Ok(pos) => blk[pos].1 = val,
            Err(pos) => blk.insert(pos, (key, val)),
        }
        if self.blocks[bi].len() > self.cap {
            let mid = self.blocks[bi].len() / 2;
            let right = self.blocks[bi].split_off(mid);
            self.blocks.insert(bi + 1, right);
        }
    }
    pub fn range_scan(&self, lo: i64, hi: i64) -> Vec<(i64, i64)> {
        let mut out = Vec::new();
        for blk in &self.blocks {
            if blk.is_empty() || blk.last().unwrap().0 < lo || blk[0].0 > hi { continue; }
            for &(k, v) in blk {
                if k >= lo && k <= hi { out.push((k, v)); }
            }
        }
        out
    }
}
