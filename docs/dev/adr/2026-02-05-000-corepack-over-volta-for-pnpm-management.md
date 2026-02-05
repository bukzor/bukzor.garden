# Corepack over volta for pnpm management

**Date:** 2026-02-05
**Status:** Accepted

## Context

The repo needs pnpm for wrangler (Cloudflare deploy) and future JS tooling.
Both local dev and GHA CI need a reproducible way to get the right pnpm version.
The `packageManager` field in package.json pins the version; the question is what
reads that field and provides pnpm.

## Decision

Use **corepack** (ships with Node.js) to manage pnpm. Pin the version via
`packageManager` in package.json.

- Local dev: `brew "node"` in Brewfile provides corepack. Conditional on
  corepack not already being present (developer may have node via volta, nvm, etc.).
- GHA: runners already have node. Just `corepack enable && pnpm install`.
- Brewfile is not reused in GHA — `brew bundle` is too slow. GHA uses dedicated
  actions instead.

## Alternatives Considered

### Volta
- **Pros:** Manages both node and pnpm. Single tool. Already used on some dev machines.
- **Cons:** Extra tool download (node+volta vs just node). GHA runners already have
  node, so volta adds overhead to manage what's already there. Loses on network
  weight, GHA compat, and cold-to-warm time.

### Brew node + brew pnpm
- **Pros:** Simple.
- **Cons:** No per-project version pinning. Extra moving parts with no clear benefit.
  Rejected early.

### Conditional Brewfile reuse in GHA
- **Pros:** Single source of truth for dependencies.
- **Cons:** `brew bundle` is slow in CI. Speed cost outweighs the DRY benefit.

## Consequences

**Positive:**
- One fewer tool to install (no volta needed for this repo)
- GHA setup is minimal: `corepack enable`
- Version pinning via standard `packageManager` field

**Negative:**
- Corepack is technically experimental (but stable in practice, and volta is no less experimental)
- Two places declare deps: Brewfile (local) and GHA workflow (CI)

## Related

- Related to: `.claude/todo.kb/2026-02-05-000-deploy-cloudflare-pages.md`
