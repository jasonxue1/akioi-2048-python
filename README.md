# akioi-2048 (Python)

Python package for a 2048 variant with multiplier tiles and score tracking.

## Features

- Classic 2048 mechanics on a 4×4 board
- Multiplier tiles: -1 (×1), -2 (×2), -4 (×4)
- Simple functional API (`init`, `step`) for simulations/AI
- Detects victory (65536 tile) and game over

## Install

```bash
pip install akioi-2048
```

Supports Python 3.8–3.13.

## Quick Start

```python
import akioi_2048 as ak

board = ak.init()  # 4x4 board with two starting tiles
next_board, delta, state = ak.step(board, ak.Direction.Down)

print("delta=", delta)
print("state=", state)
print("next=", next_board)
```

## Usage

- Import enums from the package and drive the game with a simple loop.
- `step` returns the next board, the score delta for that move, and the current state.

```python
import random
import akioi_2048 as ak

def pretty(board: list[list[int]]) -> str:
    return "\n".join("\t".join(f"{v:>5}" for v in row) for row in board)

board = ak.init()
total = 0

while True:
    # pick a random direction (replace with your policy/AI)
    direction = random.choice([
        ak.Direction.Up, ak.Direction.Down,
        ak.Direction.Left, ak.Direction.Right,
    ])

    board, delta, state = ak.step(board, direction)
    total += delta

    print(f"move={direction.name:>5}  delta={delta:>6}  total={total:>7}")
    print(pretty(board), "\n")

    if state is ak.State.Victory:
        print("You win!")
        break
    if state is ak.State.GameOver:
        print("Game over.")
        break
```

Notes:

- Spawns are random after valid moves; runs are non-deterministic by design.
- Directions must be `ak.Direction.{Up,Down,Left,Right}`; states are `ak.State.{Victory,GameOver,Continue}`.

## API

- `init() -> list[list[int]]`
  - Create a new board with two starting tiles.

- `step(board: list[list[int]], direction: Direction) -> tuple[list[list[int]], int, State]`
  - Apply one move. If the board changes, a new tile appears in a random empty cell.
  - Returns `(new_board, delta_score, state)` with `state` in `{State.Victory, State.GameOver, State.Continue}`.
  - `direction` must be `ak.Direction.{Up,Down,Left,Right}`.

### Tiles and Scoring

- Positive numbers are normal tiles (2, 4, 8, …).
  - Merging equal positives adds them; score increases by the merged value.
  - Two 65536 tiles do not merge.
- Negative numbers are multipliers: `-1` = ×1, `-2` = ×2, `-4` = ×4.
  - `-1 + -1 -> -2` (score −2)
  - `-2 + -2 -> -4` (score −4)
  - `-4` tiles cannot merge further
- Number × Multiplier merges produce a number: e.g. `512 + -2 -> 1024` (score +1024)
  - Constraint: they must be adjacent in the move direction,
    and the lower/forward tile must have no empty cells beyond it (per rules).

### Spawning

A new tile spawns after a valid move with probabilities: `2`, `4`, `-1`, `-2`.
Two tiles spawn at game start.

### States

- Victory: a `65536` tile exists
- GameOver: no legal moves remain
- Continue: otherwise

## Rules Reference

Detailed rules and examples live under `rules/`.

## Development

Run tests locally (requires Python >=3.8):

```bash
uv venv .venv
source .venv/bin/activate
uv run maturin develop
uv run pytest
```
