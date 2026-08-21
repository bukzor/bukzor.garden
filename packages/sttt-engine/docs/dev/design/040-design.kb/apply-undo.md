---
why:
    - ../030-requirements.kb/move-application.md
    - ../030-requirements.kb/no-allocation-hot-path.md
---

# Apply/Undo Pattern

Moves are applied in-place with an undo record for reversal.

## Apply

1. Set bit in current player's cell mask
2. Update sub-board status if move caused win/draw
3. Update `closed` mask if sub-board status changed
4. Update meta-board status if needed
5. Compute new `next_board` from local cell index
6. Switch current player

## Undo Record

Captures only what changed:
- Previous `next_board`
- Move index (which bit to clear)
- Previous sub-board status (if changed)
- Previous meta-board masks (if changed)

## Undo

Restore state from undo record. Exact inverse of apply.

## Alternative: Copy

For simple use cases, `apply_copy` clones state then applies. Less efficient but simpler for consumers who don't need undo.
