# Hopcroft-Karp O(E sqrt(V)) Maximum Bipartite Matcher in Rust (`hopcroft_karp_bip_rs`)

> Category: **Graph Matching** | Language: **Rust 2021**

## Overview

Computes maximum cardinality bipartite matching by finding shortest augmenting path layers via BFS and DFS.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(E * sqrt(V)) layered BFS + vertex-disjoint DFS augmenting paths` |
| **Space Complexity** | `O(V + E)` |

## Mathematical & Safety Invariants

- Matched left and right vertices form a valid bijection of maximum cardinality.

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
