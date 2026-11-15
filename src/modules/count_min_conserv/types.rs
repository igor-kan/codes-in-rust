#[derive(Debug, Clone)]
pub struct SketchConfig {
    pub width: usize,
    pub depth: usize,
    pub seed: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConservativeSketch {
    pub width: usize,
    pub depth: usize,
    pub seed: u64,
    pub rows: Vec<Vec<u64>>,
    pub total_mass: u64,
}
