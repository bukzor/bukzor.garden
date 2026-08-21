---
why:
    - ../040-design.kb/state-representation.md
    - ../040-design.kb/legal-move-generation.md
    - ../040-design.kb/apply-undo.md
---

# Game

The complete game state.

## Fields

```rust
pub struct Game {
    x_cells: u128,      // X occupancy (bits 0..80)
    o_cells: u128,      // O occupancy (bits 0..80)
    x_won: u16,         // sub-boards won by X (bits 0..8)
    o_won: u16,         // sub-boards won by O (bits 0..8)
    closed: u16,        // closed sub-boards (bits 0..8)
    next_board: i8,     // constraint (-1 = any)
    current: Mark,      // whose turn
}
```

## Interface

**Inspection:**
- `current_player() -> Mark`
- `cell(index: u8) -> Option<Mark>`
- `board_status(board: u8) -> BoardStatus`
- `status() -> BoardStatus` — meta-board status
- `constraint() -> Option<u8>` — required board, if any

**Move generation:**
- `legal_moves() -> impl Iterator<Item = Move>`
- `legal_moves_mask() -> u128`
- `is_legal(Move) -> bool`

**Mutation:**
- `apply(Move) -> Undo`
- `undo(Undo)`
- `apply_copy(Move) -> Game` — convenience, clones first

**Construction:**
- `Game::new() -> Game` — initial empty state
- `Game::from_moves(impl Iterator<Item = Move>) -> Game`
