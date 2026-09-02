# Checkout is our markup linking out, not an embedded Stripe surface

**Date:** 2026-09-02
**Status:** Accepted

## Context

The support footer's shape changed four times in two days: three buttons,
then one button, then an embedded `<stripe-pricing-table>`, then two
buttons. That churn is what this ADR exists to stop.

The embed was adopted to put prices on the page instead of behind a
click -- a real want. What it actually rendered: two of the three SKUs
(a customer-chosen price cannot appear in a pricing table at all), the
annual prices amortized into `$0.08 per month` headlines directly above
a note arguing that larger gifts go further, the product name and blurb
repeated once per column, and a white Stripe-blue card against a sepia
page whose visual anchor is meant to be the board.

Stripe's dashboard can restyle a pricing table with a brand color and a
font. It cannot add the missing SKU, un-amortize the headline, or
deduplicate the columns.

## Decision

The page owns the pitch, the amounts, and the buttons. Stripe owns the
checkout that opens after the click, and nothing this side of it. One
Payment Link per SKU we choose to offer; no Stripe-rendered markup, and
no third-party script on the page.

## Alternatives Considered

### Embedded pricing table
- **Pros:** prices visible without a click, no per-SKU Payment Links to maintain
- **Cons:** drops the customer-chosen SKU; amortizes annual prices to
  per-month; repeats product copy per column; unstylable past a brand
  color; loads `js.stripe.com` for every visitor including the ones who
  never support the game; makes the deployed checkout unverifiable by
  `curl`, so the liveness gate goes blind

### Our markup, linking out (chosen)
- **Pros:** amounts visible without a click *and* in our own type; every
  SKU offerable; page stays self-contained; the grep gate keeps working
- **Cons:** each offered SKU needs its own Payment Link; the amounts live
  in two places (our markup and Stripe), so they can drift

## Consequences

**Positive:**
- The page is again reachable-and-checkable by `curl`; the liveness gate
  (`grep -c -e PLACEHOLDER -e 'stripe.com/test_' -e 'pk_test_'`) means
  something once more.
- No third-party JS.

**Negative:**
- Price drift is now possible: an amount edited in Stripe does not update
  the button label. The reconciler in
  `.claude/todo.kb/2026-09-02-000-stripe-config-as-code.md` is the place
  that should eventually catch it.

**Neutral:**
- Which SKUs appear on the page is now a page-level choice, not whatever
  Stripe decides to render. Today: `$6 a year` and `Any amount`; the
  $1/yr price stays live in the account but off the page.

## Related

- Supersedes: the embed adopted in `ba1dc9f` (same day)
- Related to: `private.bukzor-llc/strategy.kb/products.kb/sttt.md` (fee
  analysis: why an annual SKU exists at all)
