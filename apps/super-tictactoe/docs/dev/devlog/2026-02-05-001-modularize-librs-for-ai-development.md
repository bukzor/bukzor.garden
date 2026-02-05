# Devlog: Modularize lib.rs for AI development

## Focus

Split monolithic `lib.rs` (~530 lines) into modules in preparation for minimax
AI implementation. Pure refactor — no behavior changes.

## What happened

Extracted three modules from `lib.rs`:
- `game.rs` — pure game types and logic (Mark, Board, Game, win detection)
- `ai_random.rs` — random move picker (extracted from `Ui::pick_random`)
- `ui.rs` — DOM rendering, event handling, auto-play scheduling

`lib.rs` is now a thin entry point (~50 lines) holding just `App` and module
declarations.

## Decisions

### Module naming: `ai_random.rs` not `ai.rs`

**Rationale:** Names the strategy, not the category. When minimax arrives it
becomes `ai_minimax.rs` — parallel files with clear purpose.

### Visibility: pub by default, non-pub for internal helpers

**Rationale:** User wants Python-style "everything accessible, no warranty" for
V0. However, `pub` items in a lib crate suppress dead-code warnings, so internal
DOM helpers in `ui.rs` (render functions, `cell_from_event`, `update_constraints`,
`find_cell`) are kept non-pub to preserve that feedback.

### Dependency direction: game ← ai, game ← ui ← lib

**Rationale:** `game.rs` has zero imports (pure logic). `ai_random.rs` only
depends on `js_sys` for entropy. All DOM coupling lives in `ui.rs`. Clean
layering that keeps game logic testable without web dependencies.

## Conventions Established

- Module files named by what they *do*, not their category
- `pub` for cross-module API, non-pub for module-internal helpers
- Game logic stays dependency-free (no web_sys, no DOM)

## Open Questions

- Will `ai_random.rs` survive as-is, or get absorbed into a broader `ai` module
  once minimax lands?
