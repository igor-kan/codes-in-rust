# Count-Min Sketch with Conservative Update Frequency Estimator (`count_min_conserv`)

> Category: **Streaming & Probabilistic Sketches** | Language: **Rust 2021**

## Overview

Implements Estan-Varghese conservative-update Count-Min Sketch in safe Rust using SplitMix64 salted hashes, strictly preventing over-inflation of untouched heavy-hitter counters during collisions.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(d) update & query for d pairwise-independent hash rows` |
| **Space Complexity** | `O(d * w) 64-bit counters` |

## Mathematical & Safety Invariants

- Every frequency query estimate satisfies estimate(x) >= true_count(x) (never underestimates).
- Conservative update guarantees point estimates never exceed standard Count-Min estimates.

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
