# AGENTS
## Project Overview

- Rules are under rules directory.

## Development Workflow

- Install `mado` using
  `cargo install --git https://github.com/akiomik/mado mado` before making
  changes.
- Install prettierd using `npm install -g @fsouza/prettierd` before making changes.
- Run `cargo check`, `uv tool run ruff check .`, and `mado check .` before pushing.
- Run `uv run maturin develop` followed by `uv run pytest` to execute the full test suite.

## Notes

- Tests use the `uv` tool to manage Python dependencies.
