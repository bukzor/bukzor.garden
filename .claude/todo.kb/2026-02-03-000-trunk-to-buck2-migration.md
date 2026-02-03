<anthropic-skill-ownership llm-subtask />

# Trunk to Buck2 Migration

**Priority:** Low (deferred until after V0)
**Complexity:** High
**Context:** `docs/private.bukzor-llc/strategy.kb/tech-stack.md`

## Problem Statement

Strategic tech stack specifies Buck2 as the build system, but current implementation uses Trunk. Need a migration path that doesn't violate "products before infrastructure" principle.

## Current Situation

- Super Tic-Tac-Toe uses Trunk for Rust → WASM builds
- Trunk provides: WASM compilation, asset bundling, dev server with hot reload
- Global install: `cargo install trunk`
- V0 not yet shipped

## Proposed Solution

**Phase 0: Ship V0 with Trunk** (now)
Keep Trunk. Ship Super Tic-Tac-Toe. Collect friction data.

**Phase 1: Buck2 Hello World** (after 1st game ships)
Parallel Buck2 setup that can build the same WASM output. Don't replace Trunk yet.

**Phase 2: Buck2 for 2nd Game** (Worm)
Build Worm with Buck2 from the start. Keep Trunk for TTT.

**Phase 3: Migrate TTT** (after 2nd game ships)
Port TTT to Buck2. Remove Trunk dependency.

## Implementation Steps

### Phase 0: Complete V0 (current)

- [ ] Finish super-tictactoe with Trunk
- [ ] Ship to Cloudflare Pages
- [ ] Document Trunk pain points as they arise

### Phase 1: Buck2 Bootstrap

- [ ] Install Buck2, verify it runs
- [ ] Create minimal BUCK file that compiles a Rust crate
- [ ] Add wasm32-unknown-unknown target support
- [ ] Reproduce Trunk's WASM output with Buck2
- [ ] Document: what does Buck2 give us that Trunk doesn't?

### Phase 2: Buck2 for Worm

- [ ] Build Worm entirely with Buck2
- [ ] Solve dev server / hot reload (Buck2 + browser-sync? custom?)
- [ ] Compare DX: Buck2 vs Trunk
- [ ] Decide: is migration worth it?

### Phase 3: Migrate TTT (conditional)

- [ ] Port TTT build to Buck2
- [ ] Remove Trunk from project
- [ ] Update HACKING.md

## Open Questions

- Does Buck2 have good WASM toolchain support, or do we need custom rules?
- What's the dev server story? Trunk's hot reload is valuable.
- Is the complexity justified for 3 small games, or is Buck2 for "later" scale?

## Success Criteria

- [ ] All games build with Buck2
- [ ] Dev experience comparable to Trunk (hot reload or equivalent)
- [ ] Single `buck2 build` command produces deployable artifacts
- [ ] No global tool installs required (Buck2 manages toolchains)

## Notes

From constitution: "Easy to disappear for 18 months building tooling"

This migration is explicitly **after** V0. If Buck2 setup takes more than a day, pause and reassess. The goal is leverage across many products—if we only ship 3 games, Trunk is fine.

Risk: Buck2's Rust/WASM ecosystem may be immature. Evaluate before committing.
