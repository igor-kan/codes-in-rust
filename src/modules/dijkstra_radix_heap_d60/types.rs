#[derive(Clone, Copy, Debug)]
pub struct WEdge {
    pub to: usize,
    pub cost: u64,
}

pub const MODULE_TAG: &str = "dijkstra_radix_heap_d60";
