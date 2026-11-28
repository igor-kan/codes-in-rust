# Myers Bit-Parallel Levenshtein Distance Kernel (`myers_bit_edit_rs_d61`)

> Category: **String Metrics** | Language: **Rust 2021**

## Overview

Computes exact Levenshtein edit distance by propagating match, positive, and negative bit vectors over the text bytes and falling back to row DP beyond 64 bytes.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N * ceil(M/64)) word-parallel edit distance` |
| **Space Complexity** | `O(M/64 + 256) bit masks` |

## Mathematical & Safety Invariants

- Distance equals zero exactly when pattern and text are identical.

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
