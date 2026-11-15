use crate::errors::HllError;
use crate::interface::CardinalityEstimator;
use crate::types::HllSketch;

#[inline]
fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

#[inline]
pub fn alpha_m(m: usize) -> f64 {
    match m {
        16 => 0.673,
        32 => 0.697,
        64 => 0.709,
        _ => 0.7213 / (1.0 + 1.079 / (m as f64)),
    }
}

impl CardinalityEstimator for HllSketch {
    fn observe(&mut self, item: u64) {
        let h = splitmix64(item);
        let p = self.precision;
        let idx = (h >> (64 - p)) as usize;
        let rem = (h << p) | (1u64 << (p - 1));
        let rho = (rem.leading_zeros() as u8) + 1;
        if rho > self.registers[idx] {
            self.registers[idx] = rho;
        }
    }

    fn estimate(&self) -> f64 {
        let m = self.registers.len() as f64;
        let mut inv_sum = 0.0f64;
        let mut zeros = 0usize;
        for &r in &self.registers {
            inv_sum += 2.0f64.powi(-(r as i32));
            if r == 0 {
                zeros += 1;
            }
        }
        let raw = alpha_m(self.registers.len()) * m * m / inv_sum;
        if raw <= 2.5 * m && zeros > 0 {
            m * (m / (zeros as f64)).ln()
        } else {
            raw
        }
    }

    fn merge(&mut self, other: &Self) -> Result<(), HllError> {
        if self.precision != other.precision {
            return Err(HllError::InvalidDimension("precision mismatch during HLL merge"));
        }
        for (a, &b) in self.registers.iter_mut().zip(other.registers.iter()) {
            if b > *a {
                *a = b;
            }
        }
        Ok(())
    }
}
