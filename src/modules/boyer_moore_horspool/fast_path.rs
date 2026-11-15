use crate::types::HorspoolTable;

pub fn count_matches(tbl: &HorspoolTable, haystack: &[u8]) -> usize {
    tbl.find_all(haystack).len()
}
