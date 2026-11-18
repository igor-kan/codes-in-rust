# Vose O(1) Alias Table Discrete Categorical Sampler in Rust (`vose_alias_table_rs`)

> Category: **Randomized Algorithms** | Language: **Rust 2021**

## Overview

Constructs Vose's O(1) Alias Table from arbitrary unnormalized weights using small/large worklists.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N) table construction + O(1) bucket draw` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Every bucket mixes at most two outcomes and conserves exact categorical probabilities.

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
