---
managed-by: Skill(llm-subtask)
cost-benefit-sweh:
  timebox:
    "@value": 3.0
    rationale: |
      Phase A (manual wrangler) done. Phase B (GitHub Actions auto-
      deploy) ~1.5h. Phase C (Pulumi infra) ~1.5h, optional. Decide
      on the fly how far to go.
    confidence: tentative
  benefit-2w:
    "@value": 0.5
    rationale: |
      Once auto-deploy lands: every push ships. Multiplier on
      super-tictactoe iteration speed. ~$50 of "no manual deploy
      friction" in window.
    confidence: tentative
  cost-of-delay-2w:
    "@value": 0.1
    rationale: |
      Phase A works manually; nothing's blocked by deferring Phase
      B/C. The friction tax is small per-2w but compounds with
      every push. Tagged "V0 blocker" in the file, but
      mechanically not blocking — just slow.
    confidence: tentative
---

# Deploy to Cloudflare Pages

<!-- rationale: private.bukzor-llc/public/technical-principles.kb/version-controlled-operations.md -->
<!-- tooling: private.bukzor-llc/strategy.kb/tech-stack.md -->

**Priority:** High (V0 blocker for super-tictactoe)
**Scope:** Repo-wide — applies to all apps

## Graduated Plan

Each phase is a superset of the previous. Each works standalone. Decide on the
fly how far to take it.

### Phase A: Script + Wrangler CLI ✓

Minimum viable deploy. Manual invocation, fully reproducible.

- [x] Install wrangler: `pnpm add -D wrangler` (repo-pinned, not global)
- [x] Create CF Pages project via `wrangler pages project create`
- [x] Verify public URL works (`bukzor-garden--super-tictactoe.pages.dev`)
- [x] Document in `apps/super-tictactoe/docs/dev/deploy.md`

### Phase B: GitHub Actions

Automated deploy on push to main.

- [ ] GHA workflow: build + deploy using `cloudflare/wrangler-action`
- [ ] Handle monorepo: only deploy when app files change
- [ ] Wrangler API token as GitHub secret

### Phase C: Pulumi for Infrastructure

Declarative infrastructure. CF Pages project defined in code.

- [ ] App-scoped Pulumi project (e.g. `apps/super-tictactoe/infra/`)
- [ ] CF Pages project as Pulumi resource
- [ ] Shared infra (if any) in repo-scoped project, kept minimal
- [ ] GHA deploys infra changes via Pulumi, app changes via wrangler

## Decisions

- **One CF Pages project per app.** Shared nothing. Aligns with design-for-throwaway.
- **Wrangler pinned via pnpm** (`pnpm add -D wrangler`). Repo-local, locked in pnpm-lock.yaml.
- **Custom domain is independent.** Low effort (~10 min), do whenever. Ship on `*.pages.dev` first.
  - `bukzor.garden` registered 2026-09-02 -- the Stripe statement descriptor now
    names it, so pointing it at the app is no longer purely cosmetic.
- **Pulumi projects are app-scoped.** Each app controls its own infra. Shared infra minimized.

## Success Criteria

- [x] super-tictactoe accessible at a public URL
- [x] Deploy is a two-command procedure, documented
- [x] Procedure is fully reproducible from repo contents alone
