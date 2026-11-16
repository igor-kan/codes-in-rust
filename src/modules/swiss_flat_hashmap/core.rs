use crate::interface::FlatDict;
use crate::types::{FlatSlot, SwissFlatMap, CTRL_DELETED, CTRL_EMPTY};

#[inline]
pub fn mix64(mut k: u64, seed: u64) -> u64 {
    k ^= seed;
    k = (k ^ (k >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    k = (k ^ (k >> 27)).wrapping_mul(0x94d049bb133111eb);
    k ^ (k >> 31)
}

#[inline]
fn h2_byte(h: u64) -> u8 {
    ((h >> 57) as u8) & 0x7F
}

impl SwissFlatMap {
    fn rehash(&mut self, new_cap: usize) {
        let old_ctrl = std::mem::replace(&mut self.ctrl, vec![CTRL_EMPTY; new_cap]);
        let old_slots = std::mem::replace(&mut self.slots, vec![FlatSlot { key: 0, val: 0 }; new_cap]);
        self.len = 0;
        self.tombstones = 0;
        for (idx, c) in old_ctrl.into_iter().enumerate() {
            if c < 0x80 {
                let s = &old_slots[idx];
                self.insert(s.key, s.val);
            }
        }
    }
}

impl FlatDict for SwissFlatMap {
    fn insert(&mut self, key: u64, val: i64) -> Option<i64> {
        let cap = self.ctrl.len();
        if (self.len + self.tombstones + 1) * 4 >= cap * 3 {
            let next_cap = if self.len * 2 >= cap { cap * 2 } else { cap };
            self.rehash(next_cap);
        }
        let mask = self.ctrl.len() - 1;
        let h = mix64(key, self.seed);
        let tag = h2_byte(h);
        let mut idx = (h as usize) & mask;
        let mut step = 1usize;
        let mut first_tomb: Option<usize> = None;
        loop {
            let c = self.ctrl[idx];
            if c == tag && self.slots[idx].key == key {
                let old = self.slots[idx].val;
                self.slots[idx].val = val;
                return Some(old);
            }
            if c == CTRL_DELETED && first_tomb.is_none() {
                first_tomb = Some(idx);
            } else if c == CTRL_EMPTY {
                let dst = first_tomb.unwrap_or(idx);
                if self.ctrl[dst] == CTRL_DELETED {
                    self.tombstones -= 1;
                }
                self.ctrl[dst] = tag;
                self.slots[dst] = FlatSlot { key, val };
                self.len += 1;
                return None;
            }
            idx = (idx + step) & mask;
            step += 1;
        }
    }

    fn get(&self, key: u64) -> Option<i64> {
        let mask = self.ctrl.len() - 1;
        let h = mix64(key, self.seed);
        let tag = h2_byte(h);
        let mut idx = (h as usize) & mask;
        let mut step = 1usize;
        loop {
            let c = self.ctrl[idx];
            if c == tag && self.slots[idx].key == key {
                return Some(self.slots[idx].val);
            }
            if c == CTRL_EMPTY {
                return None;
            }
            idx = (idx + step) & mask;
            step += 1;
        }
    }

    fn remove(&mut self, key: u64) -> Option<i64> {
        let mask = self.ctrl.len() - 1;
        let h = mix64(key, self.seed);
        let tag = h2_byte(h);
        let mut idx = (h as usize) & mask;
        let mut step = 1usize;
        loop {
            let c = self.ctrl[idx];
            if c == tag && self.slots[idx].key == key {
                self.ctrl[idx] = CTRL_DELETED;
                self.len -= 1;
                self.tombstones += 1;
                return Some(self.slots[idx].val);
            }
            if c == CTRL_EMPTY {
                return None;
            }
            idx = (idx + step) & mask;
            step += 1;
        }
    }
}
