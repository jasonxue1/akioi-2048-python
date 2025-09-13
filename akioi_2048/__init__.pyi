from enum import Enum

class Direction(Enum):
    Up: "Direction"
    Down: "Direction"
    Left: "Direction"
    Right: "Direction"

class State(Enum):
    Victory: "State"
    GameOver: "State"
    Continue: "State"

def step(
    board: list[list[int]], direction: Direction
) -> tuple[list[list[int]], int, State]:
    """Apply one move.

    If the board changes, a new tile appears in a random empty cell.

    Args:
        board: 4x4 game board. Positive numbers are normal tiles (2, 4, 8,
            ...). Negative numbers are multipliers: -1=x1, -2=x2, -4=x4
            (absolute value is the multiplier).
        direction: Move direction enum: ``Direction.{Up,Down,Left,Right}``.

    Returns:
        ``(new_board, delta_score, state)`` where ``state`` is ``State``.

    Note:
        If the board does not change, no tile is spawned and ``delta_score=0``.
    """

def init() -> list[list[int]]:
    """Create a new board with two starting tiles.

    Returns:
        Fresh board ready for play.
    """
