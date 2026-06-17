# Bevy 2048

A small 2048 clone implemented with Rust and Bevy.

The game uses real-time rendering primitives for the board, tiles, rounded panels, modal dialog, and buttons. It does not load images or spritesheets; text uses bundled Clear Sans font files under `assets/fonts`.

## Run

```bash
cargo run
```

## Web

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```

For production builds:

```bash
trunk build --release
```

The checked-in `.cargo/config.toml` enables the `getrandom` Web backend for `wasm32-unknown-unknown`, which is required by `rand` in browser builds.

## Controls

- Arrow keys or `WASD`: move tiles
- Touch swipe: move tiles on touchscreen devices
- `R` or the `New Game` button: restart the game

## Features

- Standard 4x4 2048 board
- Random `2` and `4` tile spawning
- Correct single-merge-per-move behavior
- Score and best score display
- Win state after reaching `2048`
- Game-over modal with restart and close-game buttons
- Per-tile slide animation from each source cell into its final cell after each valid move
- Touch-friendly swipe controls and new-game button
- Input throttling prevents rapid repeated moves from pushing tile visuals outside the board
- Reference-inspired cream and warm-taupe 2048 palette with rounded panels and tiles
- Clear Sans font rendering for the original 2048-like typography

## Code Structure

- `src/main.rs`: Binary entry point; only creates the Bevy `App`, adds `Game2048Plugin`, and runs it
- `src/lib.rs`: Top-level `Game2048Plugin`, window setup, default Bevy plugins, app-level system ordering
- `src/game.rs`: `GamePlugin`, 2048 board state, movement/merge rules, score updates, win/game-over checks, and unit tests
- `src/input.rs`: `InputPlugin`, keyboard/touch input mapping, restart handling, and move-input throttling
- `src/render.rs`: `RenderPlugin`, camera setup, Clear Sans font loading, Bevy UI layout, rounded board/tiles, modal dialog, and button handling
- `.cargo/config.toml`: WebAssembly-specific Rust flags for browser randomness support
- `index.html`: Trunk entry point for the WebAssembly build
- `Trunk.toml`: Web build configuration
- `.github/workflows/ci.yml`: PR/push checks for formatting, clippy, tests, and debug builds
- `.github/workflows/web.yml`: WebAssembly build and GitHub Pages deployment from `main`
- `.github/workflows/release.yml`: Tagged desktop release builds for Linux, macOS, and Windows

## Verify

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check
```

## Release

Create a version tag to build desktop release artifacts and attach them to a GitHub Release:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The release workflow packages the executable, `assets/`, and `README.md` for Linux, Windows, Intel macOS, and Apple Silicon macOS.
