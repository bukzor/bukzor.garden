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
    - [x] Set up Stripe account -- live 2026-09-02; sole proprietor,
      DBA `bukzor.garden`
      (`private.bukzor-llc/strategy.kb/merchant-identity.md`)
    - [x] Configure the SKUs -- product `Support bukzor.garden`
      (`prod_VBgL3XU4rNdIN0`) with three prices: $1/yr, $6/yr, and a
      customer-chosen one-off. Replaces the original $0.50/mo (2026-09-02);
      Stripe does not support customer-chosen amounts on recurring prices,
      so the variable option is necessarily one-off
    - [ ] Decide the one-off preset. Set 2026-09-02: preset $1, minimum
      $0.50, maximum $50. Open (agent-drafted, vetoable): raise the preset
      to ~$5. $6/yr is the default radio and $1/yr already serves "less",
      so the open box is most likely reached by someone wanting to give
      *more* -- whom a $1 preset answers with a dollar. Nets $0.67 against
      $4.56. Counter: a low preset is friendlier and FEE_OPTIMIZE says
      wait for data
    - [x] Add button to page -- `<footer class="support">` in index.html,
      `.support*` rules in style.css, both hrefs still `PLACEHOLDER`
    - [x] Swap the `PLACEHOLDER` hrefs for the real Payment Link -- one
      link, not three: all three prices ride a single `donate.stripe.com`
      link, with `Default` picking the initial radio selection. Footer
      collapsed to one button accordingly
    - [ ] Confirm the link is *active*, then deploy
      -- gate: `grep -c -e PLACEHOLDER -e 'stripe.com/test_' index.html`
      must print `0` (it does), AND the link must not serve a deactivated
      checkout. (Liveness clause agent-drafted 2026-09-02, vetoable: a
      paused link returns HTTP 200 and passes every grep -- on 2026-09-02
      this one served `inactive`/`deactivated` in the body while looking
      perfectly healthy to a status-code check.)
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
