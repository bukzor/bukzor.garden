# HTML+CSS rendering for Super Tic-Tac-Toe

**Date:** 2026-02-03
**Status:** Accepted

## Context

Super Tic-Tac-Toe needs to render a 9x9 grid (3x3 meta-grid of 3x3 sub-boards). Need to choose between DOM-based rendering or Canvas.

## Decision

Use HTML elements + CSS Grid, manipulated via `web_sys`.

## Alternatives Considered

### HTML+CSS (DOM + CSS Grid)
- **Pros:** Simple for discrete grids, click handling via DOM events, no new dependencies
- **Cons:** Less suited for animations, potential performance limits at scale

### Canvas
- **Pros:** Full control, better for animations, consistent rendering
- **Cons:** Manual hit detection, more code for a static grid, overkill for TTT

## Consequences

**Positive:**
- Simpler implementation for a turn-based grid game
- DOM events provide free click/touch handling

**Negative:**
- Worm (next product) will likely need Canvas anyway for continuous movement

**Neutral:**
- Per `strategy.kb/sequencing.md`, "wasteful OK" for first product
- Hearts may share DOM approach, giving two data points for potential extraction

## Related

- Related to: `strategy.kb/v0-definition.md` ("canvas or DOM"), `strategy.kb/sequencing.md`
