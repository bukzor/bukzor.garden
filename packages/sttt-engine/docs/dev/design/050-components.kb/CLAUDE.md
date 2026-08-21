--- # workaround: anthropics/claude-code#13003
requires:
    - Skill(llm.kb)
---

# Components

Concrete types and modules that implement the design.

## What Belongs

- Type definitions and their responsibilities
- Public interfaces (how to use)
- Key implementation choices
- Data layout and algorithm details

## What Does NOT Belong

- Design rationale (lives in 040-design.kb/)
- Build/deploy details (lives in 060-deliverables.kb/)
- Cross-cutting policies (lives in technical-policy.kb/)

## When to Update

- When adding new types
- When interfaces change
- When implementation details are worth documenting
