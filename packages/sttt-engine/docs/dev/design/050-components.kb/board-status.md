---
why:
    - ../040-design.kb/state-representation.md
    - ../040-design.kb/win-masks.md
---

# BoardStatus

Status of a sub-board or the meta-board.

## Definition

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoardStatus {
    Open,
    Won(Mark),
    Drawn,
}
```

## Notes

- `Open` — game/sub-board still in play
- `Won(Mark)` — three-in-a-row achieved
- `Drawn` — all cells filled, no winner

A sub-board is "closed" if `Won(_)` or `Drawn`.
