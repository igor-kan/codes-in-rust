# Lexicographically Minimal Kahn DAG Topological Sorter (`topological_kahn_rs`)

> Category: **Graph Algorithms** | Language: **Rust 2021**

## Overview

Computes lexicographically smallest deterministic topological ordering of directed graphs via min-heap Kahn queue peeling, returning explicit cycle obstruction nodes when acyclic constraints fail.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O((V + E) log V)` |
| **Space Complexity** | `O(V + E)` |

## Mathematical & Safety Invariants

- Every directed edge (u, v) satisfies pos[u] < pos[v] across emitted ordering.
- Output ordering is the unique lexicographically minimum valid topological order.

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
