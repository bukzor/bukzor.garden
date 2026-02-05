# Devlog: Unit tests with mutation testing validation

## Focus

Add unit tests for `game.rs` and validate them via mutation testing. User pushed
back on unverified tests: "I don't trust the tests until I see them fail, the
right way."

## What happened

Wrote 19 unit tests covering Mark, SubBoard, and Game logic. Then ran the full
mutation testing workflow — planned 11 mutations based on the implementation,
injected each bug, verified tests caught it, reverted.

All 11 mutations caught. Tests earn their keep.

## Decisions

### Mutation testing over coverage metrics

**Rationale:** Line coverage tells you code was executed, not that tests would
catch regressions. Mutation testing proves tests detect specific bugs. Takes
longer but provides real confidence.

### Track mutations in `.kb/` directory

**Rationale:** Each mutation is a file with status (todo/done/gap) and injection
details. Future sessions can extend or reference. Follows the llm.kb pattern for
structured knowledge.

### No trait abstraction for AI strategies

**Rationale:** User asked about shared trait between `ai_random.rs` and future
`ai_minimax.rs`. Decided against — two strategies with one call site doesn't
earn a trait. Difficulty slider will blend at the call site, not via dispatch.

## Mutations Tested

All `status: done` in `docs/dev/mutation-testing.kb/`:
- mark-next-returns-same
- anti-diagonal-same-as-diagonal
- check-winner-skips-equality
- sub-board-play-skips-outcome-update
- sub-board-allows-overwrite
- sub-board-allows-play-after-win
- game-constraint-targets-source-board
- game-never-frees-constraint
- game-skips-meta-win-detection
- legal-moves-includes-resolved-boards
- sub-board-draw-not-detected

## Conventions Established

- Mutation testing validates test suites before trusting them
- `mutation-testing.kb/` tracks what's been validated
- Tests go in inline `#[cfg(test)] mod tests` in the same file

## Open Questions

- Should mutation testing be part of CI, or manual spot-checks?
- When minimax arrives, do we mutation-test `ai_minimax.rs` the same way?
