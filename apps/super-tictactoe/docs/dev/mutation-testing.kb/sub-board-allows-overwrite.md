---
status: done
---

`SubBoard::play` doesn't check if a cell is already occupied. Marks silently overwrite each other.

## Injection

```rust
// In SubBoard::play(), remove:
// if self.cells[row][col] != Mark::Empty {
//     return false;
// }
```

## Test Coverage

Caught by:
- `sub_board_rejects_occupied_cell`
