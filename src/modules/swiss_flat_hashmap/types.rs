pub const CTRL_EMPTY: u8 = 0x80;
pub const CTRL_DELETED: u8 = 0xFE;

#[derive(Debug, Clone)]
pub struct FlatSlot {
    pub key: u64,
    pub val: i64,
}

#[derive(Debug, Clone)]
pub struct SwissFlatMap {
    pub ctrl: Vec<u8>,
    pub slots: Vec<FlatSlot>,
    pub len: usize,
    pub tombstones: usize,
    pub seed: u64,
}
