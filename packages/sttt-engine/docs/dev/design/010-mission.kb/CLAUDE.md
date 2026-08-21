--- # workaround: anthropics/claude-code#13003
requires:
    - Skill(llm.kb)
---

# Mission

Why this project exists. The problem being solved, who benefits, what success looks like.

## What Belongs

- Project purpose and motivation
- Target audience and their needs
- Success criteria (qualitative)
- Scope boundaries

## What Does NOT Belong

- Goals (those decompose from mission, live in 020-goals.kb/)
- Technical decisions (those live in 040-design.kb/ or ADRs)
- Metrics (those are requirements in 030-requirements.kb/)

## When to Update

- Rarely. Mission should be stable.
- If scope changes fundamentally
- If motivation needs clarification
