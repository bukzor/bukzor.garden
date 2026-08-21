---
why:
    - ../020-goals.kb/testability.md
    - ../030-requirements.kb/legal-moves.md
    - ../030-requirements.kb/win-detection.md
---

# Tests

Unit and property tests to verify correctness.

## Location

- `src/*.rs` — inline unit tests (`#[cfg(test)]`)
- `tests/` — integration tests (if needed)

## Key Test Categories

**Invariants:**
- apply then undo restores exact state
- legal moves are subset of empty cells
- legal moves respect constraint

**Win detection:**
- each of 8 patterns detected for sub-boards
- each of 8 patterns detected for meta-board
- draw detected when all boards closed, no winner

**Edge cases:**
- constraint sends to closed board → any open board
- last move fills board without win → drawn
- meta-board win ends game immediately

## Running

```bash
cargo test              # all tests
cargo test -- --nocapture  # with output
```
