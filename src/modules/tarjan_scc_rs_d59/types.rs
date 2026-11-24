#[derive(Debug, Clone)]
pub struct SccSummaryRs {
    pub count: usize,
    pub comps: Vec<usize>,
}

pub const MODULE_TAG: &str = "tarjan_scc_rs_d59";
