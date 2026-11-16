use crate::types::PalindromeProfile;

pub fn total_palindromic_substrings(p: &PalindromeProfile) -> usize {
    let odd_sum: usize = p.d_odd.iter().sum();
    let even_sum: usize = p.d_even.iter().sum();
    odd_sum + even_sum
}
