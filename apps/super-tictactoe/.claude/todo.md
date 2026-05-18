---
managed-by: Skill(llm-subtask)
---

- [x] GameBuilder: use `BoardPos`/`CellPos` instead of flat indices (0-8)

- [ ] V0 milestones
  - [x] Deploy — [Phase A complete](../../.claude/todo.kb/2026-02-05-000-deploy-cloudflare-pages.md), live at `bukzor-garden--super-tictactoe.pages.dev`
  - [ ] Theme selector — one vanity SKU (e.g. X/O colors)
  - [ ] AI opponent — minimax
    - [x] Auto-play checkboxes (test harness)
    - [ ] Smarter AI — minimax ([design rationale](../../docs/dev/devlog/2026-02-06-000-minimax-ai-design.md))
      - [x] Add `enum_map` crate + derive `Enum` on `Mark`
      - [x] Add `Grid<T>` typedef (`[[T; 3]; 3]`), update SubBoard/Board/check_winner
      - [x] Add `count_corners(grid, to_mark) -> EnumMap<Mark, usize>`
      - [x] Add `count_threats(grid, to_mark) -> EnumMap<Mark, usize>`
      - [x] Add `Score` tuple + `evaluate()` in ai_minimax.rs
      - [ ] Add `search(game, depth, α, β, maximizing, player) -> Score`
        - Alpha-beta pruning, calls score() at leaves/depth=0
      - [ ] Add `pick_best(game) -> Move`
        - Calls search, random pick among moves with equal best score
      - [ ] Wire ai_minimax into lib.rs
        - src/ai_minimax.rs has test specs ready; implement the `todo!()` stubs
      - [ ] Integration — replace pick_random in auto-play
    - [ ] Difficulty slider — smooth adjustment between random and optimal play
  - [ ] Support button — "$0.50/mo" external checkout
    - [ ] Research & choose payment platform (Stripe, Ko-fi, GitHub Sponsors, etc.)
    - [ ] Set up account + configure $0.50/mo product
    - [ ] Add button to page
  - [ ] Auto-deploy via GHA — `cloudflare/wrangler-action` on push to main

## Later

- [ ] Sticky inputs — persist auto-play/delay settings via localStorage
- [ ] Persist game state across refresh (localStorage)
- [ ] Undo button
- [ ] Restart button
- [ ] How-to-play hint — collapsible rules explanation for new players
- [ ] Win celebration — visual payoff on meta-game win
- [ ] Score tracking — session win counter across games
- [ ] Revisit turn-indicator UI — consider integrating auto-play controls
- [ ] Tablet layout — improve UI at tablet size, portrait and landscape
- [ ] Fix vertical gap glitch — 1-3px gap between sub-board cell borders
- [ ] Unit tests
- [ ] Revisit board styling (beyond classic lines-on-white)
- [ ] Immutable game state — `play` returns new state instead of mutating, cleaner for minimax
  - Can then cache/precompute `.outcome` on construction
