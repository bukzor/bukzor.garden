---
status: done
---

`SubBoard::play` doesn't check `is_full()` after ruling out a winner. Full boards with no winner stay `InProgress` instead of becoming `Draw`.

## Injection

```rust
// In SubBoard::play(), remove:
// } else if self.is_full() {
//     self.outcome = Outcome::Draw;
// }
```

## Test Coverage

Caught by:
- `sub_board_draw`
