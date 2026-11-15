#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HllConfig {
    pub precision: u8,
    pub dense_bias_correction: bool,
}

#[derive(Debug, Clone)]
pub struct HllSketch {
    pub precision: u8,
    pub registers: Vec<u8>,
}
