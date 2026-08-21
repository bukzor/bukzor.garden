---
why:
    - ../030-requirements.kb/legal-moves.md
    - ../030-requirements.kb/no-allocation-hot-path.md
---

# Move Representation

A move is a single byte: `u8` in range 0..80.

This is the global cell index on the 9x9 board, row-major order.

## Rationale

- Tiny and copyable (no heap)
- Natural for bitset operations
- Symmetry transforms become table lookups
- Derives `(board, cell)` when needed: `board = (r/3)*3 + (c/3)`, `cell = (r%3)*3 + (c%3)`

## Conversions

```
index -> (row, col): r = i/9, c = i%9
(row, col) -> index: i = 9*r + c
index -> (board, cell): via (row, col)
(board, cell) -> index: r = (b/3)*3 + (c/3), c = (b%3)*3 + (c%3)
```
