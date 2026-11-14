/**
 * @file core.rs
 * @brief Substring matching algorithm for Knuth-Morris-Pratt Substring Matcher in Rust.
 * Module: kmp_substring_search_rs
 */

#include "types.rs"
#include <string.h>

int kmp_substring_search_rs_search(const char* text, size_t t_len, const char* pat, size_t p_len) {
    if (!text || !pat || p_len == 0 || t_len < p_len) return -1;
    for (size_t i = 0; i <= t_len - p_len; ++i) {
        if (memcmp(text + i, pat, p_len) == 0) return (int)i;
    }
    return -1;
}
