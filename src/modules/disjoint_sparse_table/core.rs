use crate::types::DisjointSparseTable;

impl DisjointSparseTable {
    pub fn new_mul_mod(arr: &[u64], modulus: u64) -> Self {
        let orig_len = arr.len();
        let pad_len = orig_len.next_power_of_two().max(2);
        let levels = pad_len.trailing_zeros() as usize;
        let mut data = vec![1u64; pad_len];
        for (i, &x) in arr.iter().enumerate() {
            data[i] = x % modulus;
        }
        let mut table = vec![vec![1u64; pad_len]; levels];
        for h in 0..levels {
            let block = 1usize << (h + 1);
            let half = block >> 1;
            let mut start = 0usize;
            while start < pad_len {
                let mid = start + half;
                table[h][mid - 1] = data[mid - 1];
                if mid >= start + 2 {
                    for i in (start..(mid - 1)).rev() {
                        table[h][i] = ((data[i] as u128 * table[h][i + 1] as u128) % modulus as u128) as u64;
                    }
                }
                table[h][mid] = data[mid];
                for i in (mid + 1)..(start + block) {
                    table[h][i] = ((table[h][i - 1] as u128 * data[i] as u128) % modulus as u128) as u64;
                }
                start += block;
            }
        }
        Self { orig_len, pad_len, levels, modulus, data, table }
    }

    #[inline]
    pub fn query_inclusive(&self, l: usize, r: usize) -> u64 {
        if l == r {
            return self.data[l];
        }
        let diff = l ^ r;
        let lvl = (usize::BITS - 1 - diff.leading_zeros()) as usize;
        ((self.table[lvl][l] as u128 * self.table[lvl][r] as u128) % self.modulus as u128) as u64
    }
}
