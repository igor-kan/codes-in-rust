pub fn count_occurrences_fast(text: &[u8], pattern: &[u8]) -> usize {
    crate::core::find_all_z(text, pattern).len()
}
