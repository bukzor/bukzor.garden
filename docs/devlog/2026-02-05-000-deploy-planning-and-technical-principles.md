# Deploy Planning & Technical Principles

## 2026-02-05: Pivoted from doing to documenting

**Intent:** Execute deploy of super-tictactoe to Cloudflare Pages.

**Pivot:** Realized the deploy plan involved decisions (graduated approach,
tooling, infra-as-code) that span the repo and enterprise. Shifted to
documenting the plan and its rationale before executing.

**Created: `private.bukzor-llc/public/`**

Staging area for a future public bukzor-llc repo. Triggered by realizing that
technical principles referenced from OSS repos should themselves be public.
The `public/` subdirectory simulates the eventual repo; paths break loudly
when migration happens.

**Created: `technical-principles.kb/`**

Started with one principle (version-controlled-operations) to support the
deploy plan. Expanded when we noticed constitutional principles
(design-for-throw-away, guard-your-attention, better-never-perfect) had
engineering expressions that deserved public, standalone documentation.

Structure that emerged: two axioms (`write-it-down`, `formalize-later`) and
three derived principles (`version-controlled-operations`, `shared-nothing`,
`design-for-throwaway`). The axiom/derived relationship is documented in the
collection's CLAUDE.md for future maintainers.

**Created: graduated deploy plan**

Three-phase plan (script+wrangler → GHA → Pulumi), each phase a superset of
the previous, each standalone. All open questions resolved during the session:
one CF Pages project per app (shared-nothing), wrangler pinned via pnpm,
app-scoped Pulumi projects, custom domain independent of deploy.

**Decisions:**

- Technical principles separated from playbook.kb/ because they're public-facing
- Principle filenames use command-form assertions (opinionated, slightly overstated)
- Public files must not reference private content — stand alone with engineering rationale
- Maintenance conventions documented immediately as they're clarified
