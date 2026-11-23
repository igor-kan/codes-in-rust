# Suffix Array & Kasai Linear LCP Builder (`lcp_kasai_rs_d58`)

> Category: **Suffix Indexing** | Language: **Rust 2021**

## Overview

Constructs lexicographic suffix permutation and adjacent Longest Common Prefix array via Kasai O(N) amortized rank propagation.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N log^2 N) SA + O(N) Kasai LCP` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Distinct substrings formula N*(N+1)/2 - sum(lcp) matches exact count cleanly.

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
