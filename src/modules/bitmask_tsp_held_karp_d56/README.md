# Held-Karp Exact TSP Bitmask Dynamic Program (`bitmask_tsp_held_karp_d56`)

> Category: **NP-Hard Exact Solvers** | Language: **Rust 2021**

## Overview

Finds exact shortest Hamiltonian tour lengths visiting every city cleanly once via subset-mask dynamic programming.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N^2 * 2^N)` |
| **Space Complexity** | `O(N * 2^N)` |

## Mathematical & Safety Invariants

- Exact TSP tour beats or equals canonical sequential cycle sum.

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
