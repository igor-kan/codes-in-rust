use crate::types::{LruSlab, Slot};
use std::hash::Hash;

impl<K: Eq + Hash + Clone, V> LruSlab<K, V> {
    pub fn len(&self) -> usize {
        self.map.len()
    }

    fn detach(&mut self, idx: usize) {
        let (p, n) = {
            let s = self.entries[idx].as_ref().expect("live slot");
            (s.prev, s.next)
        };
        match p {
            Some(pi) => self.entries[pi].as_mut().expect("prev").next = n,
            None => self.head = n,
        }
        match n {
            Some(ni) => self.entries[ni].as_mut().expect("next").prev = p,
            None => self.tail = p,
        }
    }

    fn push_front(&mut self, idx: usize) {
        let old_head = self.head;
        {
            let s = self.entries[idx].as_mut().expect("live slot");
            s.prev = None;
            s.next = old_head;
        }
        if let Some(oh) = old_head {
            self.entries[oh].as_mut().expect("old head").prev = Some(idx);
        }
        self.head = Some(idx);
        if self.tail.is_none() {
            self.tail = Some(idx);
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let idx = *self.map.get(key)?;
        self.detach(idx);
        self.push_front(idx);
        self.entries[idx].as_ref().map(|s| &s.val)
    }

    pub fn put(&mut self, key: K, val: V) -> Option<(K, V)> {
        if let Some(&idx) = self.map.get(&key) {
            self.entries[idx].as_mut().expect("slot").val = val;
            self.detach(idx);
            self.push_front(idx);
            return None;
        }
        let mut evicted = None;
        if self.map.len() == self.cap {
            let tail_idx = self.tail.expect("nonempty tail");
            self.detach(tail_idx);
            let old = self.entries[tail_idx].take().expect("tail slot");
            self.map.remove(&old.key);
            self.free.push(tail_idx);
            evicted = Some((old.key, old.val));
        }
        let idx = if let Some(fi) = self.free.pop() {
            self.entries[fi] = Some(Slot { key: key.clone(), val, prev: None, next: None });
            fi
        } else {
            let ni = self.entries.len();
            self.entries.push(Some(Slot { key: key.clone(), val, prev: None, next: None }));
            ni
        };
        self.map.insert(key, idx);
        self.push_front(idx);
        evicted
    }
}
