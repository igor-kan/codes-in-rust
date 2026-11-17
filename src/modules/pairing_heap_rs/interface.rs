use crate::errors::PairingHeapError;
pub trait IndexedMinHeap {
    fn push(&mut self, key: i64, val: usize) -> usize;
    fn decrease_key(&mut self, id: usize, new_key: i64) -> Result<(), PairingHeapError>;
    fn pop_min(&mut self) -> Option<(i64, usize)>;
}
