---
why:
    - ../030-requirements.kb/legal-moves.md
    - ../030-requirements.kb/no-allocation-hot-path.md
---

# Legal Move Generation

Legal moves are computed as a bitmask, not a list.

## Algorithm

1. Determine target region:
   - If `next_board >= 0` and that board is open: `BOARD_MASK[next_board]`
   - Otherwise: union of `BOARD_MASK[b]` for all open boards

2. Mask out occupied cells:
   - `legal = region & !(x_cells | o_cells)`

## Precomputed Tables

- `BOARD_MASK[9]: u128` — cells belonging to each sub-board
- `OPEN_REGION[512]: u128` — cells in open boards, indexed by `closed` mask

## Iteration

Iterate set bits: `while bits != 0 { i = bits.trailing_zeros(); bits &= bits - 1; ... }`
