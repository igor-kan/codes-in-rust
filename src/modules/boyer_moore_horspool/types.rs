#[derive(Debug, Clone)]
pub struct HorspoolTable {
    pub needle: Vec<u8>,
    pub shift: [usize; 256],
}
