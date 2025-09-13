import pytest

import akioi_2048 as ak


def test_step_rejects_non_power_of_two() -> None:
    board = [
        [3, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError, match=r"^invalid tile value: 3$"):
        ak.step(board, ak.Direction.Down)


def test_step_rejects_too_large_value() -> None:
    board = [
        [131072, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError, match=r"^invalid tile value: 131072$"):
        ak.step(board, ak.Direction.Down)


def test_step_rejects_unknown_negative_multiplier() -> None:
    board = [
        [-3, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError, match=r"^invalid tile value: -3$"):
        ak.step(board, ak.Direction.Down)


def test_step_rejects_one() -> None:
    board = [
        [1, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError, match=r"^invalid tile value: 1$"):
        ak.step(board, ak.Direction.Down)
