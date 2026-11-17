# Arena-Indexed Meldable Pairing Heap with O(1) Decrease-Key (`pairing_heap_rs`)

> Category: **Priority Queues** | Language: **Rust 2021**

## Overview

Implements Fredman-Sedgewick pairing heap over safe index-based Vec arena nodes supporting handle-indexed decrease-key and two-pass merge cleanup without raw pointers.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(1) push/decrease_key, O(log N) amortized pop_min` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Heap partial-order key[parent] <= key[child] holds after every push, decrease_key, and pop_min.

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
