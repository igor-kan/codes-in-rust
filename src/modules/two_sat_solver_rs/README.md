# Implication-Graph 2-SAT Tarjan SCC Solver in Rust (`two_sat_solver_rs`)

> Category: **Graph & Logic** | Language: **Rust 2021**

## Overview

Solves Boolean 2-CNF formulas by building 2N-literal implication graphs, decomposing SCCs with Tarjan DFS, and checking variable/negation separability.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(V + M) linear SCC + truth assignment` |
| **Space Complexity** | `O(V + M)` |

## Mathematical & Safety Invariants

- Every clause evaluates true under returned witness whenever formula is satisfiable.

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
