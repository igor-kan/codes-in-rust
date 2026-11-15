# Cache-Compact Fenwick Binary Indexed Tree with Binary Lifting Order Search (`fenwick_prefix_rs`)

> Category: **Prefix & Range Structures** | Language: **Rust 2021**

## Overview

Maintains dynamic cumulative frequencies over 1..N with LSB-bit isolation (i & i.wrapping_neg()), linear O(N) initialization, and single-pass O(log N) prefix lower_bound lifting.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N) linear build, O(log N) add / sum / lower_bound` |
| **Space Complexity** | `O(N) 1-based flat i64 array` |

## Mathematical & Safety Invariants

- prefix_sum(i) equals exact naive sum(arr[0..i]) for all 0 <= i <= n.
- find_prefix_lower_bound(target) returns smallest idx where prefix_sum(idx) >= target.

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
