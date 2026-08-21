# sttt-engine Mission

## Problem

Need a clean-room implementation of Super Tic-Tac-Toe (Ultimate TTT) game logic for comparison against other implementation approaches.

## Audience

- The author, comparing implementation strategies
- Future consumers wanting a game engine library

## Success

- Correct game logic (rules, win detection, constraints)
- Clean API suitable for various consumers (UI, AI, testing)
- Performance sufficient for AI search algorithms
- Code quality enabling fair comparison

## Scope

**In scope:**
- Game state representation
- Move validation and application
- Win/draw detection
- Pure Rust library (no platform dependencies)

**Out of scope:**
- UI/rendering
- AI algorithms (separate concern)
- WASM bindings (consumer responsibility)
- Networking/multiplayer
