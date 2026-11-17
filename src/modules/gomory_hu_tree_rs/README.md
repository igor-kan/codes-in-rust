# Gusfield Gomory-Hu All-Pairs Minimum Cut Tree (`gomory_hu_tree_rs`)

> Category: **Network Flow** | Language: **Rust 2021**

## Overview

Constructs an N-vertex flow-equivalent tree where tree path bottleneck weights equal exact s-t global minimum cut capacities.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(V * T_maxflow)` |
| **Space Complexity** | `O(V + E)` |

## Mathematical & Safety Invariants

- All-pairs tree cut matches undirected K4 cut capacities cleanly.

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
