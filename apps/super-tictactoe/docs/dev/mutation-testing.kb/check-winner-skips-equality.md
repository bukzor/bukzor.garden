---
status: done
---

`check_winner` doesn't verify all three cells match — any line with a non-empty first cell is treated as a win.

## Injection

```rust
// In check_winner()
if a.is_some() {  // was: if a.is_some() && a == b && b == c
    return a;
}
```

## Test Coverage

Caught by:
- `sub_board_draw`
- `game_alternates_turns`
