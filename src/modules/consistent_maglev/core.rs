use crate::types::MaglevRouter;

fn fnv1a_salted(bytes: &[u8], seed: u64) -> u64 {
    let mut h = 0xcbf29ce484222325u64 ^ seed;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

pub fn populate_table(backends: &[&str], m: usize) -> Vec<usize> {
    let n = backends.len();
    let mut offset = Vec::with_capacity(n);
    let mut skip = Vec::with_capacity(n);
    for b in backends {
        offset.push((fnv1a_salted(b.as_bytes(), 0x1111) as usize) % m);
        skip.push(((fnv1a_salted(b.as_bytes(), 0x2222) as usize) % (m - 1)) + 1);
    }
    let mut next = vec![0usize; n];
    let mut entry = vec![usize::MAX; m];
    let mut filled = 0usize;
    while filled < m {
        for i in 0..n {
            let mut c = (offset[i] + next[i] * skip[i]) % m;
            while entry[c] != usize::MAX {
                next[i] += 1;
                c = (offset[i] + next[i] * skip[i]) % m;
            }
            entry[c] = i;
            next[i] += 1;
            filled += 1;
            if filled == m {
                break;
            }
        }
    }
    entry
}

impl MaglevRouter {
    pub fn route_key(&self, key: u64) -> usize {
        let h = fnv1a_salted(&key.to_le_bytes(), 0x9e3779b9);
        self.lookup[(h as usize) % self.table_size]
    }
}
