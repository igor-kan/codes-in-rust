#[derive(Debug, Clone)]
pub struct TspResult {
    pub min_tour: i64,
    pub vertices: usize,
}

pub const MODULE_TAG: &str = "bitmask_tsp_held_karp_d56";
