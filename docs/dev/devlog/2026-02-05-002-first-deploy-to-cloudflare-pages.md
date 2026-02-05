# Devlog: first deploy to Cloudflare Pages

## Focus

Complete Phase A of the deploy plan: create CF Pages project, deploy
super-tictactoe, document the procedure. User's first time with Cloudflare.

## What happened

Authenticated wrangler via `wrangler login`. Created CF Pages project and
deployed the built WASM app. Game is live at
`bukzor-garden--super-tictactoe.pages.dev`.

Created `apps/super-tictactoe/docs/dev/deploy.md` as the deploy reference, with
a breadcrumb from CLAUDE.md's new "How To" section.

Discussion about documentation durability: `todo.md` and `todo.kb/` are
ephemeral (cleared when done), so durable info must live in committed docs.
`deploy.md` is a convenience reference that may go stale; the CLAUDE.md
breadcrumb is what agents actually read.

## Decisions

### CF Pages naming convention: `<repo>--<app>`

**Rationale:** Namespaces projects so different repos can't conflict. Double-dash
separates repo from app; single hyphens are within-name. e.g.
`bukzor-garden--super-tictactoe`.
**Alternatives considered:** Full path from `~/repo` (too long/ugly), various
levels of truncation. Settled on repo+app as the right balance.

### No deploy script in `ops/` for now

**Rationale:** Two documented commands are sufficient at current scale. A script
adds a file to maintain for marginal benefit. GHA (Phase B) is the next
meaningful automation step.

### Deploy doc at app scope, not repo scope

**Rationale:** Only one app exists. Repo-scope conventions (like the naming
pattern) will be lifted when a second app arrives.

## Open Questions

- Will `docs/dev/deploy.md` actually get updated as deploy evolves, or will it rot?
- Is the CLAUDE.md breadcrumb sufficient to get future agents to find and load deploy docs?

## Conventions Established

- CF Pages project naming: `<repo>--<app>` with `--` separator
- Deploy docs live at `apps/<app>/docs/dev/deploy.md`
- CLAUDE.md "How To" section is the breadcrumb for operational procedures
