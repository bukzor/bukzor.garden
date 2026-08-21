---
why:
    - ../030-requirements.kb/win-detection.md
---

# Win Masks

Win detection uses precomputed bitmasks.

## Sub-board Wins

8 winning patterns per sub-board (3 rows, 3 cols, 2 diagonals).

For a 9-bit sub-board occupancy mask:
```
WIN_PATTERNS: [u16; 8] = [
    0b111_000_000,  // row 0
    0b000_111_000,  // row 1
    0b000_000_111,  // row 2
    0b100_100_100,  // col 0
    0b010_010_010,  // col 1
    0b001_001_001,  // col 2
    0b100_010_001,  // diagonal
    0b001_010_100,  // anti-diagonal
]
```

Player wins sub-board if `(occupancy & pattern) == pattern` for any pattern.

## Meta-board Wins

Same 8 patterns applied to `x_won` or `o_won` masks.

Game won if any pattern matched on meta-board.
