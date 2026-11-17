use crate::types::BasisState;
#[derive(Clone, Default)]
pub struct XorBasis { pub st: BasisState }
impl XorBasis {
    pub fn insert(&mut self, mut x: u64) -> bool {
        for bit in (0..64).rev() {
            if ((x >> bit) & 1) == 1 {
                if self.st.b[bit] == 0 { self.st.b[bit] = x; self.st.rank += 1; return true; }
                x ^= self.st.b[bit];
            }
        }
        false
    }
    pub fn max_xor(&self, seed: u64) -> u64 {
        let mut r = seed;
        for bit in (0..64).rev() { r = r.max(r ^ self.st.b[bit]); }
        r
    }
    pub fn kth(&self, k: u64) -> u64 {
        let mut d = self.st.b;
        let mut pivs = Vec::new();
        for bit in 0..64 {
            if d[bit] != 0 {
                for hi in (bit + 1)..64 { if ((d[hi] >> bit) & 1) == 1 { d[hi] ^= d[bit]; } }
                pivs.push(d[bit]);
            }
        }
        let mut out = 0u64;
        for (i, p) in pivs.into_iter().enumerate() { if ((k >> i) & 1) == 1 { out ^= p; } }
        out
    }
}
