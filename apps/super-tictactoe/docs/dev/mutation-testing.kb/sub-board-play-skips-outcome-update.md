---
status: done
---

`SubBoard::play` places the mark but never checks for wins or draws afterward. Sub-boards stay `InProgress` forever.

## Injection

```rust
// In SubBoard::play(), remove lines after cells assignment:
self.cells[row][col] = mark;
// delete: if let Some(winner) = self.check_winner() { ... }
// delete: else if self.is_full() { ... }
true
```

## Test Coverage

Caught by:
- `sub_board_row_win`
- `sub_board_col_win`
- `sub_board_diagonal_win`
- `sub_board_anti_diagonal_win`
- `sub_board_draw`
- `sub_board_rejects_play_after_win`
- `game_detects_meta_row_win`
