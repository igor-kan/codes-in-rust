# Dinic Level-Graph Blocking-Flow Solver in Rust (`dinic_maxflow_rs`)

> Category: **Network Flow** | Language: **Rust 2021**

## Overview

Computes maximum s-t network flow using Dinic BFS distance layering and current-arc pointer DFS blocking flow.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(V^2 * E) BFS level graph + current-arc DFS augmentation` |
| **Space Complexity** | `O(V + E)` |

## Mathematical & Safety Invariants

- Total augmented flow equals minimum s-t cut capacity.

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
