#[derive(Clone, Copy, Debug)]
pub struct CutStats {
    pub weight: i64,
    pub phases: usize,
}

pub const MODULE_TAG: &str = "stoer_wagner_mincut_rs";
