# Booth Minimal String Rotation Scanner (`booth_min_rotation_rs_d61`)

> Category: **String Algorithms** | Language: **Rust 2021**

## Overview

Finds the lexicographically smallest rotation of a string by advancing two candidate offsets through a doubled character buffer and skipping equal prefixes.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N) least-rotation scan with two candidate offsets` |
| **Space Complexity** | `O(N) doubled character buffer` |

## Mathematical & Safety Invariants

- The returned start index lies in [0, N) and its rotation is no greater than every other rotation.

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
