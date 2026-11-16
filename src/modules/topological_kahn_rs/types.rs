#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopoOutcome {
    Ordered(Vec<usize>),
    HasCycle(Vec<usize>),
}
