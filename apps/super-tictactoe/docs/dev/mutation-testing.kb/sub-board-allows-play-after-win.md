---
status: done
---

`SubBoard::play` doesn't check if the board is already resolved. Play continues in won/drawn boards.

## Injection

```rust
// In SubBoard::play(), remove:
// if self.outcome != Outcome::InProgress {
//     return false;
// }
```

## Test Coverage

Caught by:
- `sub_board_rejects_play_after_win`
