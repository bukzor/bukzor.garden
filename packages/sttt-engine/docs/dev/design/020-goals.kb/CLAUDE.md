--- # workaround: anthropics/claude-code#13003
requires:
    - Skill(llm.kb)
---

# Goals

How we accomplish the mission. Aspirational but achievable. Stable over project lifetime.

## What Belongs

- High-level objectives that serve the mission
- Qualities the system should have
- Constraints we've chosen to adopt

## What Does NOT Belong

- Mission itself (lives in 010-mission.kb/)
- Verifiable requirements (those live in 030-requirements.kb/)
- Implementation decisions (those live in 040-design.kb/)

## When to Update

- When mission clarification reveals new goals
- When a goal proves unachievable or irrelevant
- Rarely — goals should be stable
