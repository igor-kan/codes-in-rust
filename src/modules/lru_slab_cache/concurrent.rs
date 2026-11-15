use crate::interface::create_lru;
use crate::types::LruSlab;
use std::sync::Mutex;

pub struct ShardedLru {
    pub shards: Vec<Mutex<LruSlab<u64, u64>>>,
}

impl ShardedLru {
    pub fn new(num_shards: usize, cap_per_shard: usize) -> Self {
        let mut shards = Vec::with_capacity(num_shards);
        for _ in 0..num_shards {
            shards.push(Mutex::new(create_lru(cap_per_shard).expect("cap > 0")));
        }
        Self { shards }
    }

    pub fn put_sharded(&self, key: u64, val: u64) {
        let si = (key as usize) % self.shards.len();
        self.shards[si].lock().expect("lock").put(key, val);
    }
}
