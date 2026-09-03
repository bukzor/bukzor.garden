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
      wait for data. Strengthened 2026-09-02: the page no longer carries
      the fee note, so the preset is the only place left where the
      "one larger gift" argument can act on a visitor
    - [x] Add button to page -- `<footer class="support">` in index.html,
      `.support*` rules in style.css, both hrefs still `PLACEHOLDER`
    - [x] Swap the `PLACEHOLDER` hrefs for the real Payment Link -- one
      link, `donate.stripe.com/dRm3co3Zi6jfeV6ar81Nu00`, footer collapsed
      to one button accordingly
      -- correction (2026-09-02, `stripe payment_links list --live`): the
      claim recorded here, that all three prices rode this link behind
      radios with `Default` preselected, was false. `plink_1UBIzE5PD8pFX1kD`
      carries exactly one line item, the customer-chosen price
      (`price_1UBIzE5PD8pFX1kD0Q309OM1`, min $0.50 / preset $1 / max $50).
      The two annual prices exist but sit on no payment link, so the site
      cannot reach them. The radios belonged to the pricing table, not the
      link
    - [x] Deploy -- shipped 2026-09-02; redeployed the same day with the
      two-button footer (`aba44111`). Production serves both buttons, both
      checkout URLs answer `200`, and the gate prints `0` -- now including
      `js.stripe.com`, since the page is again free of third-party script
    - [x] Make the $6/yr SKU reachable -- it was on no payment link, so the
      site could not sell it. `plink_1UBKFN5PD8pFX1kD`
      (`buy.stripe.com/eVqeV6brKbDzbIUfLs1Nu01`) created 2026-09-02 over
      `price_1UBJ2y5PD8pFX1kD`. Footer is now two buttons, `$6 a year` and
      `Any amount`; $1/yr stays priced in the account but off the page,
      since the customer-chosen price already reaches down to $0.50
    - [x] Cut the on-page fee note (user: it added 22 words to a page that
      otherwise has six). The nudge it carried -- larger gifts net more
      against a flat 30c -- is now structural: $6/yr sits first among two
      buttons. What remains of it belongs in the preset, below
    - [x] Unpause the Payment Link -- already unpaused. Read back
      2026-09-02: `plink_1UBIzE5PD8pFX1kD` is `active: true`, and the
      account reports `charges_enabled` and `payouts_enabled` true with
      `details_submitted`. G2 (`beam-search.md`: a public URL that accepts
      money by 2026-09-26) is met on capability -- no card has been run
      through it, so end-to-end remains unproven
    - [ ] Answer the past-due identity challenge --
      `requirements.past_due` carries
      `interv_1UBJY25PD8pFX1kD.identity_verification.challenge`. Charges
      still work today; past-due requirements are what later disable an
      account. Dashboard-only (KYC-bound, per MERCHANT_SEAM)
      -- liveness gate (agent-drafted, vetoable): `grep -c -e PLACEHOLDER
      -e 'stripe.com/test_' -e 'pk_test_' index.html` must print `0`.
      Necessary, not sufficient: it proves the page points at the live
      link, not that the link accepts money
    - [x] Drop the `<stripe-pricing-table>` embed (2026-09-02, same day it
      landed). The table showed two of the three prices -- the
      customer-chosen one-off cannot appear in one -- amortized the annual
      SKUs into `$0.08 per month` headlines that contradict the fee note
      beneath them, repeated the product name and blurb once per column,
      and rendered a white Stripe-blue card against the sepia page. The
      amounts now ride the page's own copy above the one `Chip in` link,
      which restores the curl-checkable gate and the page's freedom from
      third-party JS
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
- [x] Fix vertical gap glitch -- 2.66px at worst, not the 1-3px estimated here.
  Two content-box effects: `aspect-ratio: 1` sized the *content* box, so a cell
  carrying a left border came out shorter than its col-0 neighbor; and a
  sub-board stretched by the meta-grid fed its leftover height to the auto
  rows. Fixed 2026-09-02 (`365de93`) with `box-sizing: border-box` globally
  plus `align-content: start` on `.sub-board`; worst gap 0.016px, deployed and
  verified live
  - Fallout, agent-drafted and vetoable: the board now declares its footprint
    (`--board-width`, 500px at the user's call) with fixed 8px padding, and
    `.turn-indicator`/`.support` were pointed at the same variable so the
    player panels stay flush with the board's edges. Previously they were
    400px against a board that drifted between 418 and 434 with window width
- [ ] Rethink how the grid lines are drawn. Today a line is the sum of three
  cells' borders, so its continuity is emergent -- every box-model change is a
  chance to break it, and it has broken twice. Acceptance: continuity belongs
  to the container, such that no arrangement of cell heights or box models can
  open a gap; the two weights (meta, inner) and the sub-board inset survive;
  cells keep the marks, the hover, and the hit-testing
  - Falsifiable either way: every cell's bottom edge coincides with the next
    row's top edge (tolerance 0.05px) at several viewport widths, and the
    2.66px gap that stood before `365de93` would fail it
  - Prior art in-repo: gap + container background, rejected 2026-02-03 for
    sub-pixel wobble at 1-2px. That objection is about line *width*, not
    continuity, and it predates the fixed 500px footprint
- [ ] Unit tests
- [ ] Revisit board styling (beyond classic lines-on-white)
- [ ] Immutable game state — `play` returns new state instead of mutating, cleaner for minimax
  - Can then cache/precompute `.outcome` on construction
