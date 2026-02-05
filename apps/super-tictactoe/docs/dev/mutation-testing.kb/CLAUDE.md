--- # workaround: anthropics/claude-code#13003
requires:
    - Skill(llm.kb)
---

# Mutation Testing KB

Tracks code mutations for post-hoc TDD on `src/game.rs`.

Each file represents one mutation — a specific way to break the implementation.
Files track whether tests reliably catch the bug (`done`), can't catch it (`gap`),
or haven't been tried yet (`todo`).

## What belongs here

- Mutations to game logic in `src/game.rs`
- One file per mutation, named by *how* to break the code

## What does NOT belong here

- UI/DOM mutations (those would go in a separate kb if needed)
- Test code itself (lives in `src/game.rs` `#[cfg(test)]` module)
