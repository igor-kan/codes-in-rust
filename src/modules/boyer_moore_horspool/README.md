# Boyer-Moore-Horspool Sublinear Expected-Time Byte Searcher (`boyer_moore_horspool`)

> Category: **String Algorithms** | Language: **Rust 2021**

## Overview

Searches arbitrary binary haystacks using right-to-left suffix alignment and a compact 256-byte bad-character skip table computed solely from pattern[0..M-1].

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N / M) average sublinear skips, O(N * M) worst case` |
| **Space Complexity** | `O(256) bad-character shift table` |

## Mathematical & Safety Invariants

- All returned indices satisfy haystack[i..i+M] == needle.
- Search finds identical overlapping or non-overlapping positions to brute-force scan.

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
