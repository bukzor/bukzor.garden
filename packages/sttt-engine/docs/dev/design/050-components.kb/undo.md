---
why:
    - ../040-design.kb/apply-undo.md
---

# Undo

Record for reverting a move.

## Definition

```rust
pub struct Undo {
    mov: Move,              // which cell to clear
    prev_next_board: i8,    // previous constraint
    prev_board_status: Option<(u8, BoardStatus)>,  // if sub-board status changed
    prev_meta_status: Option<BoardStatus>,         // if meta-board status changed
}
```

## Notes

Only stores what changed. Most moves only change `next_board`; sub-board wins are rare, meta-board changes rarer still.

Undo is the exact inverse of apply: clear the bit, restore cached status values.
