---
why:
    - ../030-requirements.kb/state-inspection.md
    - ../030-requirements.kb/no-allocation-hot-path.md
---

# State Representation

Game state uses bitboards for compact, fast operations.

## Cell Occupancy

- `x_cells: u128` — bit i set if X occupies cell i
- `o_cells: u128` — bit i set if O occupies cell i

Only bits 0..80 are used.

## Board Status

- `x_won: u16` — bit b set if X won sub-board b (9 bits used)
- `o_won: u16` — bit b set if O won sub-board b
- `closed: u16` — bit b set if sub-board b is closed (won or drawn)

## Constraint

- `next_board: i8` — which sub-board must be played in (-1 = any open board)

## Derived

- Current player: derived from popcount parity or stored explicitly
- Terminal status: derived from `x_won`/`o_won`/`closed` against win masks
