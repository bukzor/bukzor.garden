# Devlog: pnpm and wrangler bootstrapping

## Focus

Set up pnpm infrastructure and install wrangler, as Phase A step 1 of the
Cloudflare Pages deploy plan.

## What happened

Ported pnpm infra from `template.python-project`: `.envrc` (direnv PATH setup),
`bin/pnpm-run` (idempotent install-then-exec wrapper), `package.json` with
`packageManager` pinning.

Evaluated corepack vs volta for pnpm management. Corepack won on GHA compat
(runners already have node), network weight, and cold-to-warm time. Documented
in ADR.

While testing `brew bundle`, discovered it was rebuilding `cargo "trunk"` from
source every run. Root cause: brew sanitizes the environment, stripping
`CARGO_INSTALL_ROOT`. Cargo then couldn't find its install metadata. Fixed by
moving the install root declaration from env var to `~/.cargo/config.toml`,
which survives env sanitization. Documented in home-repo ADR.

Installed wrangler (`pnpm add -D wrangler`). Verified `pnpm-run` bootstraps
cleanly from cold (rm node_modules, pnpm-run which wrangler).

## Decisions

### Corepack over volta
**Rationale:** GHA runners already have node; corepack is zero-extra-install there.
Volta would add overhead to manage what's already present.
**ADR:** `docs/dev/adr/2026-02-05-000-corepack-over-volta-for-pnpm-management.md`

### Cargo install root in config.toml
**Rationale:** File-based config survives brew's env sanitization. Env var doesn't.
**ADR:** `~/docs/dev/adr/2026-02-05-000-cargo-install-root-via-configtoml-for-brew-bundle-compatibility.md`

## Files added/changed

- `.envrc` — direnv PATH setup (bin/, node_modules/.bin/)
- `bin/pnpm-run` — idempotent pnpm install + exec wrapper
- `package.json` — packageManager pin, wrangler devDependency
- `pnpm-lock.yaml` — lockfile
- `Brewfile` — added direnv, conditional node
- `.gitignore` — added node_modules/

## Next

- Phase A continues: `wrangler pages project create`, deploy script, verify
