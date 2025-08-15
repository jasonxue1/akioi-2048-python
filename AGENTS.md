# AGENTS

## Project Overview

- Rules are under rules directory.

## Development Workflow

- Install `mado` using
  `cargo install --git https://github.com/akiomik/mado mado` before making
  changes.
- Format Rust code with `cargo fmt --all` before committing.
- Format Python code with `ruff format`.
- Format Markdown with `npx @fsouza/prettierd`.
- Run `cargo check`, `ruff check .`, and `mado check .` before pushing.
- Run `maturin develop` followed by `pytest` to execute the full test suite.

## Notes

- Tests use the `uv` tool to manage Python dependencies.
