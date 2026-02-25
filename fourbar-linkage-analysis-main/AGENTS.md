# Project: Four-Bar Linkage Analysis

## Build & Run
- **Rust Simulator**: `cargo run` (dev) or `cargo run --release` (optimized).
- **Python Analysis**: `uv run pyscript/generate_figures.py` (requires `uv`).
- **Tests**: `cargo test` (Run single: `cargo test <test_name>`).
- **Wrapper**: `./run.sh [rust|python|build|test|clean]` handles common tasks.

## Code Style & Conventions
- **Rust**:
  - Follow standard Rust formatting (`cargo fmt`) and clippy advice.
  - Structs: PascalCase. Variables/Functions: snake_case.
  - Error Handling: Use `Result<T, String>` or `Result<T, E>`. Avoid unwrap/expect in logic.
  - Docs: Use `///` for public items. `src/fourbar.rs` contains core logic.
- **Python**:
  - Use `snake_case`. Use `pathlib` for file operations.
  - Script located in `pyscript/`.
- **General**:
  - Keep logic separated from UI (logic in `fourbar.rs`, UI in `main.rs`).
  - No existing Cursor/Copilot rules found.

## Commit Changes
- Commit changes using Conventional Commits (e.g., `<type>[optional scope]: <description>`).
