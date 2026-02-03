--- # workaround: anthropics/claude-code#13003
depends:
    - skills/llm-collab
---

# bukzor.garden — Development Guide

## Strategic Context

This repo is part of bukzor-llc. For constitution, principles, and product strategy:
`docs/private.bukzor-llc/` — see `constitution.kb/CLAUDE.md` for when to consult.

## Quick Reference

**Current tasks:** See [.claude/todo.md] for active work and priorities.

**Dev setup:** See [HACKING.md](HACKING.md)

## Architecture

Monorepo for web games. Rust → WASM via Trunk. Static hosting on Cloudflare Pages.

```
apps/           # individual games
packages/       # shared code (future)
ops/            # deployment config (future)
docs/           # architecture, devlogs, ADRs
```

## Conventions

- Keep it simple. These are calibration projects.
- No abstractions until earned (see constitution: `process.kb/abstraction-earned.md`).
- Ship fast, iterate later.
