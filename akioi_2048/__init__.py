from enum import Enum
from .akioi_2048 import init
from .akioi_2048 import step


class Direction(Enum):
    Up = "Up"
    Down = "Down"
    Left = "Left"
    Right = "Right"


class State(Enum):
    Victory = "Victory"
    GameOver = "GameOver"
    Continue = "Continue"


__all__ = ["init", "step", "Direction", "State"]
