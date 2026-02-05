<anthropic-skill-ownership llm-subtask />

# Deploy to Cloudflare Pages

<!-- rationale: private.bukzor-llc/public/technical-principles.kb/version-controlled-operations.md -->
<!-- tooling: private.bukzor-llc/strategy.kb/tech-stack.md -->

**Priority:** High (V0 blocker for super-tictactoe)
**Scope:** Repo-wide — applies to all apps

## Graduated Plan

Each phase is a superset of the previous. Each works standalone. Decide on the
fly how far to take it.

### Phase A: Script + Wrangler CLI

Minimum viable deploy. Manual invocation, fully reproducible.

- [x] Install wrangler: `pnpm add -D wrangler` (repo-pinned, not global)
- [ ] Create CF Pages project via `wrangler pages project create`
- [ ] Write deploy script in `ops/` that:
  - Runs `trunk build --release` in the app directory
  - Runs `wrangler pages deploy dist/`
- [ ] Verify public URL works (`*.pages.dev`)
- [ ] Document in HACKING.md

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
- **Pulumi projects are app-scoped.** Each app controls its own infra. Shared infra minimized.

## Success Criteria

- [ ] super-tictactoe accessible at a public URL
- [ ] Deploy is a single command (Phase A) or automatic (Phase B+)
- [ ] Procedure is fully reproducible from repo contents alone
