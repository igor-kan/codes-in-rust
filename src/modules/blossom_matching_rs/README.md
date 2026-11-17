# Edmonds Blossom General-Graph Maximum Matching in Rust (`blossom_matching_rs`)

> Category: **Graph Matching** | Language: **Rust 2021**

## Overview

Computes exact maximum matching on general non-bipartite graphs using augmenting path BFS with odd-cycle blossom LCA contraction.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(V^3) blossom contraction BFS` |
| **Space Complexity** | `O(V + E)` |

## Mathematical & Safety Invariants

- Every matched partner is mutual: match[match[u]] == Some(u).

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
