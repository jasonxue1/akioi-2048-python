# AGENTS

## Project Overview

- Rules are under rules directory.

## Run before commit

```bash
cargo clippy --all-targets --all-features -- -D warnings
uv tool run ruff check .
mado check .
uv venv .venv
source .venv/bin/activate
uv run maturin develop
uv run pytest
```
