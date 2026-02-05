# Deploy

Static WASM app on Cloudflare Pages.

**Project:** `bukzor-garden--super-tictactoe`
**URL:** https://bukzor-garden--super-tictactoe.pages.dev/

## Naming Convention

CF Pages project name: `<repo>--<app>`, e.g. `bukzor-garden--super-tictactoe`.
Double-dash separates repo from app; single hyphens are within-name.

## Manual Deploy

From repo root:

```bash
# Build
cd apps/super-tictactoe && trunk build --release

# Deploy
pnpm-run wrangler pages deploy apps/super-tictactoe/dist/ \
  --project-name bukzor-garden--super-tictactoe
```

## Auth

```bash
pnpm-run wrangler login   # one-time; opens browser OAuth flow
```

Credentials stored locally by wrangler. No secrets in the repo.

## Architecture

Trunk compiles Rust to WASM and produces a static `dist/` directory (HTML + JS + WASM).
Wrangler uploads that directory to CF Pages, which serves it as a static site.
No server-side compute. Free tier.
