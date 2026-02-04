<anthropic-skill-ownership llm-subtask />

# Refactor Object Model: Board/Game/Ui/App Separation

**Priority:** High (blocks AI opponent work)
**Complexity:** Medium
**Context:** Abandoned WIP at `archive/auto-play-wip-2026-02-04`

## Problem Statement

Current naming is irrational:
- "Board" has game rules (`current_turn`, `active_board`, `play()`)
- "Game" has UI elements (`board_el`, `final_status`)
- "App" is just a thin wasm_bindgen wrapper

Code should be named by what things represent, with attributes and methods that match.

## Current Situation

```
SubBoard: cells + outcome + play         (fine)
Game:     sub_boards + turn + constraint + outcome + play/legal_moves  (misnamed - has rules)
App:      game + board_el + status + auto_play + listener  (misnamed - has UI)
```

## Proposed Solution

Four types with rational naming:

| Type | Owns | Does |
|------|------|------|
| **SubBoard** | cells, outcome | play, check_winner (3x3 scope) |
| **Board** | sub_boards, outcome | (pure physical state) |
| **Game** | board, current_turn, active_sub_board | play, legal_moves, check_winner |
| **Ui** | game, board_el, final_status, auto_play | handle_click, schedule_auto_play |
| **App** | Rc<Ui>, _listener | wasm entry point |

Separation: **data** (Board) -> **logic** (Game) -> **UI** (Ui) -> **wasm glue** (App)

## Implementation Steps

- [ ] Rename current `Game` -> `Board`
  - [ ] `boards` -> `sub_boards`
  - [ ] Remove `current_turn`, `active_board` (will move to new Game)
  - [ ] Keep `outcome` (physical state - you can see who won by looking)
- [ ] Create new `Game` struct
  - [ ] `board: Board`
  - [ ] `current_turn: Mark`
  - [ ] `active_sub_board: Option<(usize, usize)>`
  - [ ] Move `play()`, `legal_moves()` here
- [ ] Rename current `App` -> `Ui`
  - [ ] `game: RefCell<Game>`
  - [ ] Keep `board_el`, `final_status`
  - [ ] Add `auto_play: AutoPlay` (for next step)
  - [ ] Move `handle_click()` here
- [ ] Create new `App` as wasm entry point
  - [ ] `ui: Rc<Ui>`
  - [ ] `_listener: EventListener`
  - [ ] Holds listener to keep it alive; Ui is Rc-wrapped for closures

## Success Criteria

- [ ] Each type's name matches what it contains
- [ ] `cargo check` passes
- [ ] `trunk build` succeeds
- [ ] Game still playable (manual test)

## Notes

This refactor is preparatory work for auto-play feature. The cleaner model will make it easier to add `schedule_auto_play()` to Ui without the current confusion about where game state vs UI state lives.
