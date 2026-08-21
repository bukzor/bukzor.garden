---
why:
    - ../030-requirements.kb/no-allocation-hot-path.md
---

# Benchmarks

Criterion benchmarks to verify performance requirements.

## Location

`benches/` directory, standard Cargo layout.

## Key Benchmarks

- **legal_moves** — time to generate legal move mask
- **apply_undo** — time for apply + undo cycle
- **random_playout** — time for full game from start to terminal

## Running

```bash
cargo bench                    # all benchmarks
cargo bench -- legal_moves     # specific benchmark
```

## Baseline Targets

- legal_moves: < 100ns
- apply_undo: < 200ns
- random_playout: < 50μs (for ~40 moves)

These are rough targets for AI search viability, not hard requirements.
