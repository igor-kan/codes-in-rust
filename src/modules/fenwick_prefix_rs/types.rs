#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FenwickStats {
    pub total_mass: i64,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct FenwickTree {
    pub tree: Vec<i64>,
    pub raw: Vec<i64>,
}
