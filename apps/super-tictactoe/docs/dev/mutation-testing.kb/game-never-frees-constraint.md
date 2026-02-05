---
status: done
---

`Game::play` always constrains to `(row, col)` without checking if that sub-board is resolved. Players get stuck when sent to a won/drawn board.

## Injection

```rust
// In Game::play(), change:
self.active_sub_board = Some((row, col));  // was: if target.outcome == InProgress { Some } else { None }
```

## Test Coverage

Caught by:
- `game_frees_constraint_when_target_resolved`
