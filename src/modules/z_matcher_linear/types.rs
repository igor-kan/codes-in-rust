#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchReport {
    pub offsets: Vec<usize>,
    pub longest_match: usize,
    pub overlaps_allowed: bool,
}
