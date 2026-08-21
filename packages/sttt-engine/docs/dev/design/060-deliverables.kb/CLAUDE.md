--- # workaround: anthropics/claude-code#13003
requires:
    - Skill(llm.kb)
---

# Deliverables

Build artifacts and how to produce them.

## What Belongs

- Crate/package definitions
- Build configurations
- Test and benchmark harnesses
- Release considerations

## What Does NOT Belong

- Component internals (lives in 050-components.kb/)
- Design rationale (lives in 040-design.kb/)
- Deployment of consuming apps (their concern)

## When to Update

- When adding new build targets
- When dependencies change significantly
- When build/test process changes
