pub struct BloomFilter {
    words: Vec<u64>,
    bits: usize,
    k: usize,
}

fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

impl BloomFilter {
    pub fn new(words: usize, k: usize) -> Self {
        let wc = words.max(1);
        Self { words: vec![0; wc], bits: wc * 64, k: k.max(1) }
    }
    pub fn insert(&mut self, key: u64) {
        let h1 = mix64(key);
        let h2 = mix64(key ^ 0x9e3779b97f4a7c15) | 1;
        for i in 0..self.k {
            let bit = (h1.wrapping_add((i as u64).wrapping_mul(h2)) as usize) % self.bits;
            self.words[bit >> 6] |= 1u64 << (bit & 63);
        }
    }
    pub fn may_contain(&self, key: u64) -> bool {
        let h1 = mix64(key);
        let h2 = mix64(key ^ 0x9e3779b97f4a7c15) | 1;
        for i in 0..self.k {
            let bit = (h1.wrapping_add((i as u64).wrapping_mul(h2)) as usize) % self.bits;
            if (self.words[bit >> 6] & (1u64 << (bit & 63))) == 0 {
                return false;
            }
        }
        true
    }
}
