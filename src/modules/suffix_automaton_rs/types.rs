use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct SamState {
    pub len: usize,
    pub link: Option<usize>,
    pub next: BTreeMap<u8, usize>,
}

#[derive(Debug, Clone)]
pub struct SuffixAutomaton {
    pub states: Vec<SamState>,
    pub last: usize,
    pub input_len: usize,
}
