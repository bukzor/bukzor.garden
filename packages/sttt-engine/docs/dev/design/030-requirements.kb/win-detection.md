---
why:
    - ../020-goals.kb/correctness.md
---

# Win Detection

The engine must correctly detect wins at both levels.

## Verification

- Sub-board win: three-in-a-row (8 patterns) by one player
- Meta-board win: three sub-boards won in a row by one player
- Draw: all sub-boards closed with no meta-winner
- Terminal state accurately reflects win/draw/ongoing
