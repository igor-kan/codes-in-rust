# Tarjan DFS Lowlink Bridge Edge Finder in Rust (`tarjan_bridges_lowlink`)

> Category: **Graph Connectivity** | Language: **Rust 2021**

## Overview

Identifies all cut-edges (bridges) in undirected multigraphs where low[v] > tin[u].

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(V + E) single-pass DFS discovery & lowlink cutoff` |
| **Space Complexity** | `O(V + E)` |

## Mathematical & Safety Invariants

- Removing any reported bridge disconnects the graph cleanly.

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
