use crate::types::Level;
pub struct WaveletMatrix { levels: Vec<Level>, bits_cnt: usize }
impl WaveletMatrix {
    pub fn build(data: &[u32], bits_cnt: usize) -> Self {
        let mut cur = data.to_vec();
        let mut levels = Vec::with_capacity(bits_cnt);
        for b in (0..bits_cnt).rev() {
            let mut z = Vec::new(); let mut o = Vec::new();
            let mut bits = Vec::with_capacity(cur.len());
            let mut pref = Vec::with_capacity(cur.len() + 1);
            pref.push(0);
            for &v in &cur {
                let bit = ((v >> b) & 1) as u8;
                bits.push(bit);
                pref.push(*pref.last().unwrap() + usize::from(bit == 0));
                if bit == 0 { z.push(v); } else { o.push(v); }
            }
            let zeros = z.len();
            z.extend(o);
            cur = z;
            levels.push(Level { bits, zeros, pref });
        }
        Self { levels, bits_cnt }
    }
    pub fn range_kth(&self, mut l: usize, mut r: usize, mut k: usize) -> u32 {
        let mut ans = 0u32;
        for (idx, lvl) in self.levels.iter().enumerate() {
            let l0 = lvl.pref[l]; let r0 = lvl.pref[r];
            let cnt0 = r0 - l0;
            if k < cnt0 {
                l = l0; r = r0;
            } else {
                k -= cnt0;
                ans |= 1u32 << (self.bits_cnt - 1 - idx);
                l = lvl.zeros + (l - l0);
                r = lvl.zeros + (r - r0);
            }
        }
        ans
    }
}
