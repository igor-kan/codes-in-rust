# Rubinchik-Shur Eertree Palindromic Automaton in Rust (`eertree_palindromic_rs`)

> Category: **String Automata** | Language: **Rust 2021**

## Overview

Maintains two roots (length -1 imaginary and length 0 empty) and palindrome suffix links to count all distinct palindromic substrings online in O(N).

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N) online distinct palindrome insertion & suffix-link traversal` |
| **Space Complexity** | `O(N) states` |

## Mathematical & Safety Invariants

- Number of created non-root states equals exact number of distinct palindromic substrings.

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
