# Successive Shortest Path Min-Cost Max-Flow Solver (`min_cost_max_flow_d60`)

> Category: **Network Flow** | Language: **Rust 2021**

## Overview

Routes flow units from source s to sink t while minimizing cumulative linear edge transport cost.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(F * E log V) Bellman-Ford/SPFA residual cost augmentation` |
| **Space Complexity** | `O(V + E)` |

## Mathematical & Safety Invariants

- Achieves maximum s-t flow at globally minimum total routing cost.

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
