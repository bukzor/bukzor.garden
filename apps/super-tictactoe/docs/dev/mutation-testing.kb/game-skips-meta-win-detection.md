---
status: done
---

`Game::play` doesn't check for meta-level wins after a sub-board resolves. The overall game never ends.

## Injection

```rust
// In Game::play(), remove:
// if let Some(winner) = self.check_winner() {
//     self.board.outcome = Outcome::Win(winner);
//     ...
// }
```

## Test Coverage

Caught by:
- `game_detects_meta_row_win`
