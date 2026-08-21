---
why:
    - ../040-design.kb/state-representation.md
---

# Mark

Player identity.

## Definition

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mark {
    X,
    O,
}
```

## Interface

- `opponent(self) -> Mark` — returns the other player

## Notes

No "Empty" variant. Emptiness is absence from both bitboards, not a distinct value.
