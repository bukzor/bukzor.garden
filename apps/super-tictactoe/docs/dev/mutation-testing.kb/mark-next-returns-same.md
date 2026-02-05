---
status: done
---

`Mark::next()` returns the same mark instead of alternating. Turns never switch — both players place the same symbol.

## Injection

```rust
// In Mark::next()
Mark::X => Mark::X,  // was Mark::O
```

## Test Coverage

Caught by `mark_next_alternates` and `game_alternates_turns`.
