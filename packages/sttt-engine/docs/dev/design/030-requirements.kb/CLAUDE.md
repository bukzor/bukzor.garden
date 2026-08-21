--- # workaround: anthropics/claude-code#13003
requires:
    - Skill(llm.kb)
---

# Requirements

Verifiable conditions that validate goals. Each requirement should be testable.

## What Belongs

- Specific, measurable criteria
- Conditions that can be verified by tests or inspection
- Functional and non-functional requirements

## What Does NOT Belong

- Goals (those are aspirational, live in 020-goals.kb/)
- Design decisions (those live in 040-design.kb/)
- Implementation details (those live in 050-components.kb/)

## When to Update

- When goals clarification reveals missing requirements
- When testing reveals unstated assumptions
- When a requirement proves unverifiable (reframe it)
