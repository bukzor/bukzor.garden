---
why:
    - ../020-goals.kb/correctness.md
---

# Move Application

Applying a move must produce the correct successor state.

## Verification

- The cell becomes occupied by the current player
- Turn passes to the opponent
- Next-board constraint updates based on the local cell played
- Sub-board status updates if move caused win/draw
- Meta-board status updates if sub-board status changed
