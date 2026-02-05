---
status: done
---

Anti-diagonal winning line uses `(i, i)` instead of `(i, 2-i)`. Anti-diagonal wins are never detected; diagonal wins are checked twice.

## Injection

```rust
// In winning_lines()
lines[7][i] = (i, i);  // was (i, 2 - i)
```

## Test Coverage

Caught by `sub_board_anti_diagonal_win`.
