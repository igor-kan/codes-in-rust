# Linear O(N) Manacher Odd/Even Maximal Palindrome Radii (`manacher_palindromes_rs`)

> Category: **String Searching** | Language: **Rust 2021**

## Overview

Computes exact maximal palindrome half-radii centered at every byte and between adjacent bytes via Manacher's mirrored-center window, counting all palindromic substrings in O(N).

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N)` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Every maximal radius preserves byte-reversal equality s[l..r] == rev(s[l..r]).
- Total palindromic substring count equals sum((d_odd[i] + 1)/2) + sum(d_even[i]/2).

## Module Architecture

| File | Lifecycle Role |
| :--- | :--- |
| `types.rs` | Stage 01 |
| `errors.rs` | Stage 02 |
| `interface.rs` | Stage 03 |
| `core.rs` | Stage 04 |
| `validation.rs` | Stage 05 |
| `fast_path.rs` | Stage 06 |
| `concurrent.rs` | Stage 07 |
| `invariants.rs` | Stage 08 |
| `unit_tests.rs` | Stage 09 |
| `property_tests.rs` | Stage 10 |
| `benchmark.rs` | Stage 11 |
| `README.md` | Stage 12 |

## Verification & Execution Example

```text
rustc --edition 2021 --test unit_tests.rs -o unit_bin && ./unit_bin
```

---
© 2026 Igor Kan
