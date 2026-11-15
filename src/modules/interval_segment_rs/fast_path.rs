use crate::types::Interval;

pub fn total_covered_length(coalesced: &[Interval]) -> i64 {
    coalesced.iter().map(|iv| iv.end - iv.start).sum()
}
