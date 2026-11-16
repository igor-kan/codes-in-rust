#[derive(Debug, Clone)]
pub struct AcNode {
    pub next: [usize; 256],
    pub fail: usize,
    pub outs: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct AhoAutomaton {
    pub nodes: Vec<AcNode>,
    pub pat_lens: Vec<usize>,
}
