---
status: done
---

`Game::play` sets `active_sub_board` to `(meta_row, meta_col)` (the board just played in) instead of `(row, col)` (the cell position). Constraint always points back to the same board.

## Injection

```rust
// In Game::play(), change:
self.active_sub_board = if target.outcome == Outcome::InProgress {
    Some((meta_row, meta_col))  // was (row, col)
} else {
    None
};
```

## Test Coverage

Caught by:
- `game_constrains_to_target_sub_board`
- `legal_moves_respects_constraint`
