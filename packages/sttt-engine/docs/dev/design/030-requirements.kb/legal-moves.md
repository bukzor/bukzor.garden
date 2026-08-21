---
why:
    - ../020-goals.kb/correctness.md
---

# Legal Move Generation

The engine must generate exactly the legal moves for any position.

## Verification

- Empty cells in the target board(s) are legal
- Occupied cells are never legal
- Next-board constraint is enforced (when last move dictates target)
- "Any board" rule applies when target board is closed (won or drawn)
- No legal moves exist iff game is terminal
