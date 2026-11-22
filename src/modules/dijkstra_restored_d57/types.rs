#[derive(Debug, Clone)]
pub struct PathOutcome {
    pub cost: i64,
    pub path: Vec<usize>,
}

pub const MODULE_TAG: &str = "dijkstra_restored_d57";
