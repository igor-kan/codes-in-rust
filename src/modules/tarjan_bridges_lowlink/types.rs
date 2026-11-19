#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bridge {
    pub u: usize,
    pub v: usize,
}

pub const MODULE_TAG: &str = "tarjan_bridges_lowlink";
