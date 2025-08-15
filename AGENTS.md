# AGENTS

## Project Overview

- Rules are under rules directory.

## Importance

Add of files to git by `git add .`

## Run before commit

## Format

```bash
cargo fmt --all
uv tool run ruff format
npx prettier --write "**/*.{md,yml,yaml,js,ts,json}"
taplo format "**/*.toml"
```

## Check

```bash
cargo clippy --all-targets --all-features -- -D warnings
uv tool run ruff check .
mado check .
uv venv .venv
source .venv/bin/activate
uv run maturin develop
uv run pytest
```
