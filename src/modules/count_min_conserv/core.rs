use crate::errors::SketchError;
use crate::interface::FrequencySketch;
use crate::types::ConservativeSketch;

#[inline]
pub fn hash_row(key: u64, row_idx: usize, width: usize, seed: u64) -> usize {
    let mut z = key.wrapping_add(seed).wrapping_add((row_idx as u64).wrapping_mul(0x9e3779b97f4a7c15));
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^= z >> 31;
    (z as usize) % width
}

impl FrequencySketch for ConservativeSketch {
    fn update_conservative(&mut self, key: u64, count: u64) {
        let mut indices = Vec::with_capacity(self.depth);
        let mut current_min = u64::MAX;
        for r in 0..self.depth {
            let idx = hash_row(key, r, self.width, self.seed);
            indices.push(idx);
            if self.rows[r][idx] < current_min {
                current_min = self.rows[r][idx];
            }
        }
        let target = current_min.saturating_add(count);
        for (r, &idx) in indices.iter().enumerate() {
            if self.rows[r][idx] < target {
                self.rows[r][idx] = target;
            }
        }
        self.total_mass = self.total_mass.saturating_add(count);
    }

    fn estimate(&self, key: u64) -> u64 {
        let mut best = u64::MAX;
        for r in 0..self.depth {
            let idx = hash_row(key, r, self.width, self.seed);
            if self.rows[r][idx] < best {
                best = self.rows[r][idx];
            }
        }
        best
    }

    fn merge_sketch(&mut self, other: &Self) -> Result<(), SketchError> {
        if self.width != other.width || self.depth != other.depth || self.seed != other.seed {
            return Err(SketchError::InvalidDimension("incompatible sketch geometry or seed"));
        }
        for r in 0..self.depth {
            for c in 0..self.width {
                self.rows[r][c] = self.rows[r][c].saturating_add(other.rows[r][c]);
            }
        }
        self.total_mass = self.total_mass.saturating_add(other.total_mass);
        Ok(())
    }
}
