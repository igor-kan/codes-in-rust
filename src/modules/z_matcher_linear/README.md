# Linear-Time Gusfield Z-Array Substring Matcher (`z_matcher_linear`)

> Category: **String Algorithms** | Language: **Rust 2021**

## Overview

Computes exact Longest Common Prefix Z-values over arbitrary byte slices in linear amortized operations with zero backtracking on mismatched alignments.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N + M) exact string comparisons` |
| **Space Complexity** | `O(N + M)` |

## Mathematical & Safety Invariants

- Z[i] equals exact LCP length between s[0..] and s[i..] with s[Z[i]] != s[i + Z[i]].
- Matches returned coincide 1-to-1 with naive slice windows.

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
