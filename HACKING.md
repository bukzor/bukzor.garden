# Developer Setup

```bash
rustup target add wasm32-unknown-unknown
brew bundle -v
```

The [Brewfile](Brewfile) manages dev tools including `trunk` (via `cargo "trunk"`).

## Development Workflow

```bash
# Start dev server with hot reload (from app directory)
cd apps/super-tictactoe && trunk serve

# Production build
trunk build
```

## Future Improvements

- Replace Brewfile tooling with workspace-local solution via [cargo-run-bin](https://crates.io/crates/cargo-run-bin):
  ```toml
  # Cargo.toml
  [package.metadata.bin]
  trunk = { version = "0.21" }
  ```
  Then use `cargo bin trunk` instead of global `trunk`.
