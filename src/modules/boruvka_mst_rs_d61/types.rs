#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeightedEdge {
    pub u: usize,
    pub v: usize,
    pub w: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MstSummary {
    pub weight: i64,
    pub edges: usize,
}

pub const MODULE_TAG: &str = "boruvka_mst_rs_d61";
