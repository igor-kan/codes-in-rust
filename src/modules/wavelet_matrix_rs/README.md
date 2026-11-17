# Compact Binary Wavelet Matrix (O(log V) Rank & Range K-th) (`wavelet_matrix_rs`)

> Category: **Succinct Data Structures** | Language: **Rust 2021**

## Overview

Constructs a flattened bit-level Wavelet Matrix supporting zero-pointer O(log V) range rank and K-th order-statistic queries.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N log V) build, O(log V) quantile/rank` |
| **Space Complexity** | `O(N log V)` |

## Mathematical & Safety Invariants

- Range kth agrees identically with sorting the subarray.

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
