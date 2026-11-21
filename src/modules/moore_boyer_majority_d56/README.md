# Boyer-Moore Generalized Heavy-Hitter Majority (`moore_boyer_majority_d56`)

> Category: **Streaming Frequency** | Language: **Rust 2021**

## Overview

Identifies all Misra-Gries generalized k-heavy hitters occurring strictly more than floor(N / K) times across integer slices via candidate cancellation.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N * K)` |
| **Space Complexity** | `O(K)` |

## Mathematical & Safety Invariants

- Returned elements occur strictly > len(xs)/k times in the source slice.

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
