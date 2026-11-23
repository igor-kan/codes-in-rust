#[derive(Debug, Clone, Copy)]
pub struct WeightedEdgeRs {
    pub u: usize,
    pub v: usize,
    pub w: i64,
}

pub const MODULE_TAG: &str = "kruskal_mst_rs_d58";
