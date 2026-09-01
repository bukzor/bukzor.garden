---
managed-by: Skill(llm-subtask)
cost-benefit-sweh:
  timebox:
    "@value": 8.0
    rationale: |
      V0 milestone scope: ~5h minimax (search + pick_best + integration
      + difficulty slider), ~1h theme selector, ~2h support button
      (research + setup + wire), ~0.5h GHA auto-deploy. Wall-clock
      months per session-note framing; effort hours ~8.
    confidence: tentative
  benefit-2w:
    "@value": 1.0
    rationale: |
      Hobby/learning project. Realistic 2w landing: minimax search()
      + pick_best() + integration if user puts in a focused sprint.
      Value is craft/satisfaction + experiment-completeness rather
      than dollar yield. ~1 SWEh-equivalent of forward value.
    confidence: tentative
  cost-of-delay-2w:
    "@value": 0.5
    rationale: |
      Low. No external deadline, no money flow. $0.50/mo support
      button at notional 10 supporters would be $1.25/2w forgone
      (~0.01 SWEh — negligible). Real cost is momentum/context decay
      on multi-month pauses and forgone learning velocity.
      Counter-pressure: hobby projects shouldn't outrank revenue
      paths in WSJF; 0.5 nets a reasonable floor.
    confidence: tentative
---

- [x] GameBuilder: use `BoardPos`/`CellPos` instead of flat indices (0-8)

- [ ] V0 milestones
  - [x] Deploy — [Phase A complete](../../.claude/todo.kb/2026-02-05-000-deploy-cloudflare-pages.md), live at `bukzor-garden--super-tictactoe.pages.dev`
  - [ ] Theme selector — one vanity SKU (e.g. X/O colors)
  - [ ] AI opponent — minimax
    - NB scope ruling (2026-08-30, `private.bukzor-llc/strategy.kb/products.kb/sttt.md`):
      the *good* bot is out of ship scope — 20%-time play only; minimax here is
      the modest V0 line-item, and the support button waits on none of this subtree
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
  - [ ] Support button -- external checkout, two SKUs
    - [x] Research & choose payment platform -- Stripe Payment Links (ruled
      2026-09-01; rationale, fee structure and the FEE_OPTIMIZE tripwire in
      `private.bukzor-llc/strategy.kb/products.kb/sttt.md`)
    - [ ] Set up Stripe account + configure the $0.50/mo and $6/yr products
      -- needs the user's hands (identity, bank); activation runs 1-5
      business days, so a test-mode link unblocks the wiring meanwhile
    - [x] Add button to page -- `<footer class="support">` in index.html,
      `.support*` rules in style.css, both hrefs still `PLACEHOLDER`
    - [ ] Swap the two `PLACEHOLDER` hrefs for the real Payment Links
      -- gate: `grep -c PLACEHOLDER index.html` must print `0` before
      `trunk build --release` and any `wrangler pages deploy`
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
