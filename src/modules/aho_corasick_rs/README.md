# Arena-Indexed Aho-Corasick Multi-Pattern String Automaton (`aho_corasick_rs`)

> Category: **String Searching** | Language: **Rust 2021**

## Overview

Compiles multiple byte patterns into a flat Vec trie with BFS Aho-Corasick failure links and dictionary output compression to scan arbitrary byte slices in single-pass O(N) time.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(sum(patterns) * Alphabet + Text + Matches)` |
| **Space Complexity** | `O(Trie Nodes * 256)` |

## Mathematical & Safety Invariants

- Every reported (end_offset, pattern_id) satisfies text[end - pat.len()..end] == patterns[pattern_id].
- Total unique matches equal the union of individual naive substring occurrences.

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
