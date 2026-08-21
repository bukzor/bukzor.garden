---
why:
    - ../040-design.kb/move-representation.md
---

# Move

A move is a cell index.

## Definition

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move(pub u8);
```

Newtype over `u8`. Valid range: 0..81.

## Interface

- `Move::new(index: u8) -> Move`
- `row(self) -> u8` — index / 9
- `col(self) -> u8` — index % 9
- `board(self) -> u8` — which sub-board (0..9)
- `cell(self) -> u8` — position within sub-board (0..9)

## Construction from coordinates

- `Move::from_rc(row: u8, col: u8) -> Move`
- `Move::from_bc(board: u8, cell: u8) -> Move`
