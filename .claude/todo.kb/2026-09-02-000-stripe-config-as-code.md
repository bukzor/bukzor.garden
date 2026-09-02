---
managed-by: Skill(llm-subtask)
cost-benefit-sweh:
  timebox:
    "@value": 4.0
    rationale: |
      Provider evaluation (community Terraform-bridged Stripe provider
      vs. roll-your-own against the API) ~1h. Repo-scoped project for
      account settings ~1h. App-scoped products/prices/payment-links
      for STTT ~1h. Read-back reconciler for the console-only fields
      ~1h. Phase-C-shaped: do as much as pays.
    confidence: tentative
  benefit-2w:
    "@value": 0.2
    rationale: |
      Reproducibility and drift-detection on config that is small,
      rarely changed, and already written down in
      strategy.kb/merchant-identity.md. No revenue rides on it.
    confidence: tentative
  cost-of-delay-2w:
    "@value": 0.15
    rationale: |
      Every hand-made object is one more `pulumi import` later, and
      Payment Links in particular are awkward to reshape once issued.
      Real but small, and ADOPT_SEAM already prices this as the
      expected bootstrap residue rather than debt to avoid.
    confidence: tentative
---

# Stripe configuration as code

<!-- rationale: private.bukzor-llc/web-scaling.claims.kb/enterprise-door.kb/changes-land-as-code.md -->

Bring the Stripe account under `CODE_ONLY`. Opened 2026-09-02 while
hand-configuring the account behind STTT's support button; the
identity ruling it implements is
`docs/private.bukzor-llc/strategy.kb/merchant-identity.md`.

## Import, don't author

The account is being built by hand *first*, deliberately. `ADOPT_SEAM`
(`web-scaling.claims.kb/bootstrap.kb/self-hosting-adoption.md`) covers
exactly this: hand-made resources get `pulumi import`ed afterward and
changes land as code from then on. Hand-configuring today is bootstrap
residue, not a `CODE_ONLY` violation — and the 2026-09-26 revenue date
does not survive a detour through provider evaluation.

## The seam is not where it is for Cloudflare

Stripe splits into two halves that cannot be managed the same way.

- **Declarable and appliable:** products, prices, payment links, tax
  settings, customer-portal configuration.
- **Console-only:** legal name, business name, statement descriptor,
  activation questionnaire — Stripe will not let an API key rewrite the
  account's verified identity, by design. A pricing table would land
  here too, for an unrelated reason — there is no `/v1/pricing_tables`
  endpoint at all, so one can be neither authored nor `pulumi import`ed
  — but the checkout stopped embedding one in `a6cfba7` (ADR
  `apps/super-tictactoe/docs/dev/adr/2026-09-02-000-checkout-is-our-markup-linking-out.md`,
  decided on UX and `curl`-verifiability grounds). Keep it that way
  while `CODE_ONLY` is the goal: Payment Links are API-addressable,
  a pricing table is not.

So the second half gets declared in the repo as source of truth and
**reconciled** — read back from the API, diffed, CI fails on drift —
rather than applied. That pattern generalizes to every vendor with a
verification step; naming it is the point of the llc-scope rule
MERCHANT_SEAM.

## Work

- [ ] Target a mode explicitly. Sandbox and live are separate object
      graphs, and the `stripe` CLI binds to one context at a time —
      as of 2026-09-02 it is bound to the sandbox, and `--live` is
      refused outright until `stripe switch context` selects a live
      account. Any executor must name its mode rather than inherit
      whatever the operator's CLI last pointed at; a stack that
      silently reconciles the sandbox is worse than no stack
- [ ] Decide the executor: community Terraform-bridged Stripe provider
  vs. roll-your-own against the API
  (`web-scaling.claims.kb/iac.kb/{pulumi,roll-your-own}.md`). The
  provider is not Stripe-official — weigh that against a ~200-line
  script over a config file.
- [ ] Repo-scoped Pulumi project for account-level settings — shared
  across all apps, per "shared infra minimized" in
  `2026-02-05-000-deploy-cloudflare-pages.md`
- [ ] App-scoped project for STTT's two SKUs and their Payment Links
  (`apps/super-tictactoe/infra/`)
- [ ] `pulumi import` the hand-made objects; confirm a no-op preview
- [ ] Reconciler for the console-only fields, wired into CI
- [ ] Decide where the Stripe API key lives — blocked on the llc-scope
  rules REVOKE_PATH and MERCHANT_SEAM

## Gate

A no-op `pulumi preview` against the live account is the completion
test. Until it passes, the console is still authoritative and this
item is not done.
