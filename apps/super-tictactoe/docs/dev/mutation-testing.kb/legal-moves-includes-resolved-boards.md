---
status: done
---

`Game::legal_moves` doesn't skip resolved sub-boards. Returns phantom moves in won/drawn boards.

## Injection

```rust
// In Game::legal_moves(), remove:
// if sub.outcome != Outcome::InProgress {
//     continue;
// }
```

## Test Coverage

Caught by:
- `legal_moves_skips_resolved_boards`
