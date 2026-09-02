# Devlog: 2026-09-02 — Stripe UI refine, and two records that were false

## Focus

"Help me refine the stripe integration, with respect to UI. It's pretty
rough right now," with a screenshot of the deployed page.

## Decisions

### Drop the `<stripe-pricing-table>` embed

**Rationale:** see `adr/2026-09-02-000-checkout-is-our-markup-linking-out.md`.
The screenshot was the evidence: two SKUs shown of three, `$0.08 per
month` as the page's largest number, the product blurb twice, a white
card on a sepia page.

**Alternatives considered:** restyling the table in Stripe's branding
settings, which reaches the palette and nothing else that was wrong.

### Two buttons, not one and not three

**Rationale:** the $6/yr price sat on no Payment Link, so the shipped
page could not sell the one SKU the fee analysis argues for. Created
`plink_1UBKFN5PD8pFX1kD` over `price_1UBJ2y5PD8pFX1kD` and put it first.
The $1/yr price stays live in the account but off the page: the
customer-chosen price already reaches down to $0.50, so a third button
buys clutter, not coverage.

### Cut the on-page fee note

**Rationale:** user, on the footer copy -- "it adds 22 words to my page
that otherwise has six." The note argued that one larger gift beats many
small ones. With `$6 a year` sitting first among two buttons, the
argument is structural and the prose is redundant. What survives of it
belongs in the customer-chosen preset, which is still an open decision.

## Conventions Established

- **Read the account back before writing copy about it.** Two beliefs in
  `.claude/todo.md` were false, and one of them had already been written
  into the page:
  - "All three prices ride a single link, with `Default` picking the
    initial radio" -- no. `plink_1UBIzE5PD8pFX1kD` carries exactly one
    line item, the customer-chosen price. The radios belonged to the
    pricing table; the two records got conflated.
  - "Unpause the Payment Link -- the last thing between the live page and
    money" -- the link was never paused. `active: true`, and the account
    reports `charges_enabled`, `payouts_enabled`, `details_submitted`.
    G2 (a public URL that accepts money by 2026-09-26) has been met on
    capability since before the session started, unnoticed, because the
    record said otherwise.

  Both corrections are now inline in `.claude/todo.md` next to the claims
  they overturn. The general lesson: a hand-written note about a live
  external system is a hypothesis. The Stripe CLI answers in seconds --
  `stripe payment_links list --live -d 'expand[]=data.line_items'`.

- **`stripe` CLI:** flags follow the subcommand, the `▸ Running in ...`
  banner goes to stdout and must be stripped before JSON parsing
  (`sed -n '/^{/,$p'`), and nested fields go in `-d 'a[0][b]=c'` params
  -- the bare-flag form is refused. Corrected in
  `~/.claude/must-read.kb/before/running-a-stripe-CLI-command.md`.

## Open Questions

- The customer-chosen preset ($1 today, ~$5 argued) -- now the only place
  the "larger gift" nudge can act on a visitor.
- `requirements.past_due` holds an identity-verification challenge.
  Charges work today; past-due requirements are what later disable an
  account. Dashboard-only.
- No card has been run end-to-end.

## References

- `adr/2026-09-02-000-checkout-is-our-markup-linking-out.md`
- `.claude/todo.md` -- support-button subtree, with the corrections inline
- `private.bukzor-llc/strategy.kb/products.kb/sttt.md` -- fee analysis
