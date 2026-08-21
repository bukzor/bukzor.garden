--- # workaround: anthropics/claude-code#13003
requires:
    - Skill(llm.kb)
---

# Design

Major abstractions and their relationships. How we satisfy requirements.

## What Belongs

- Key data structures and their rationale
- Relationships between abstractions
- Distilled outcomes from ADRs (not the decision journey)

## What Does NOT Belong

- Requirements (those live in 030-requirements.kb/)
- Implementation details (those live in 050-components.kb/)
- Alternative approaches (those are ADRs)

## When to Update

- After an architectural ADR is accepted
- When abstractions change significantly
- When relationships between components change
