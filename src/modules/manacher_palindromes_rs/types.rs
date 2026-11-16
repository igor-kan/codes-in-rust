#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PalindromeProfile {
    pub d_odd: Vec<usize>,
    pub d_even: Vec<usize>,
    pub longest_start: usize,
    pub longest_len: usize,
}
