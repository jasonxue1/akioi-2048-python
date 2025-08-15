# AGENTS

## Project Overview

- Rules are under rules directory.

## Development Workflow

- Run `cargo clippy`, `uv tool run ruff check .`, and `mado check .` before pushing.
- Run `source .venv/bin/activate` then `uv run maturin develop` followed by `uv run pytest` to execute the full
  test suite.
