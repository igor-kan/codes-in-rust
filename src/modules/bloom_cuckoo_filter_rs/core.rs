use crate::errors::CuckooError;
use crate::types::CuckooFilter;

#[inline]
fn splitmix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

impl CuckooFilter {
    #[inline]
    fn indices(&self, key: u64) -> (usize, usize, u16) {
        let h = splitmix(key ^ self.seed);
        let mut fp = (h & 0xFFFF) as u16;
        if fp == 0 { fp = 1; }
        let mask = self.num_buckets - 1;
        let i1 = ((h >> 16) as usize) & mask;
        let i2 = self.alt_index(i1, fp);
        (i1, i2, fp)
    }

    #[inline]
    pub fn alt_index(&self, idx: usize, fp: u16) -> usize {
        let fh = splitmix(fp as u64 ^ 0x5555_5555) as usize;
        (idx ^ fh) & (self.num_buckets - 1)
    }

    pub fn insert(&mut self, key: u64) -> Result<(), CuckooError> {
        let (i1, i2, mut fp) = self.indices(key);
        for slot in 0..4 {
            if self.buckets[i1][slot] == 0 {
                self.buckets[i1][slot] = fp;
                self.count += 1;
                return Ok(());
            }
        }
        for slot in 0..4 {
            if self.buckets[i2][slot] == 0 {
                self.buckets[i2][slot] = fp;
                self.count += 1;
                return Ok(());
            }
        }
        let mut cur = i1;
        let mut kick_rng = key ^ 0xABCDEF;
        for _ in 0..500 {
            kick_rng = splitmix(kick_rng);
            let slot = (kick_rng as usize) & 3;
            std::mem::swap(&mut fp, &mut self.buckets[cur][slot]);
            cur = self.alt_index(cur, fp);
            for s in 0..4 {
                if self.buckets[cur][s] == 0 {
                    self.buckets[cur][s] = fp;
                    self.count += 1;
                    return Ok(());
                }
            }
        }
        Err(CuckooError::CapacityFull)
    }

    pub fn contains(&self, key: u64) -> bool {
        let (i1, i2, fp) = self.indices(key);
        self.buckets[i1].contains(&fp) || self.buckets[i2].contains(&fp)
    }

    pub fn delete(&mut self, key: u64) -> bool {
        let (i1, i2, fp) = self.indices(key);
        for s in 0..4 {
            if self.buckets[i1][s] == fp {
                self.buckets[i1][s] = 0;
                self.count -= 1;
                return true;
            }
        }
        for s in 0..4 {
            if self.buckets[i2][s] == fp {
                self.buckets[i2][s] = 0;
                self.count -= 1;
                return true;
            }
        }
        false
    }
}
