use crate::errors::SamError;
use crate::types::{SamState, SuffixAutomaton};
use std::collections::BTreeMap;

pub fn build_sam(data: &[u8]) -> Result<SuffixAutomaton, SamError> {
    let mut sam = SuffixAutomaton {
        states: vec![SamState { len: 0, link: None, next: BTreeMap::new() }],
        last: 0,
        input_len: 0,
    };
    for &b in data {
        sam.extend(b);
    }
    Ok(sam)
}
