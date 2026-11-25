# Boyer-Moore-Horspool Substring Byte Matcher with Bad-Char Shifts (`boyer_moore_horspool_d60`)

> Category: **String Matching** | Language: **Rust 2021**

## Overview

Precomputes a 256-entry bad-character jump table so mismatches near the end of the pattern advance the cursor across multiple bytes.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N / M) average sublinear scan, O(N * M) worst case` |
| **Space Complexity** | `O(256) shift table` |

## Mathematical & Safety Invariants

- Produces the exact same match index list as naive sliding comparison.

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
