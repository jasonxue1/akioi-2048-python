# Copyright (c) 2024 akioi
"""Tests for :func:`akioi_2048.step`."""

import pytest

import akioi_2048 as ak

BOARD_SIZE = 4
VALID_STATES = (-1, 0, 1)


def test_step_smoke() -> None:
    """Initialize a board and perform a simple step."""
    board = ak.init()
    new_board, delta, msg = ak.step(board, 0)
    assert len(new_board) == BOARD_SIZE
    assert all(len(r) == BOARD_SIZE for r in new_board)
    assert isinstance(delta, int)
    assert msg in VALID_STATES


def test_number_merges_and_positive_score() -> None:
    """Numbers merge and increase the score."""
    board = [
        [2, 0, 0, 0],
        [2, 0, 0, 0],
        [4, 0, 0, 0],
        [4, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board[3][0] == 8
    assert new_board[2][0] == 4
    assert delta == 12


def test_multiplier_merges_and_negative_score() -> None:
    """Multipliers merge and decrease the score."""
    board = [
        [-1, 0, 0, 0],
        [-1, 0, 0, 0],
        [-2, 0, 0, 0],
        [-2, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board[3][0] == -4
    assert new_board[2][0] == -2
    assert delta == -6


def test_no_merge_for_negative_four() -> None:
    """Negative four tiles do not merge."""
    board = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [-4, 0, 0, 0],
        [-4, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board == board
    assert delta == 0


def test_down_move_without_merge() -> None:
    """Down move shifts tiles without merging."""
    board = [
        [-1, 2, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board[3][0] == -1
    assert new_board[3][1] == 2
    assert delta == 0


def test_number_and_multiplier_do_not_merge_without_tiles_below() -> None:
    """Number and multiplier stay separate without tiles below."""
    board = [
        [2, 0, 0, 0],
        [-2, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board[2][0] == 2
    assert new_board[3][0] == -2
    assert delta == 0


def test_number_and_multiplier_no_merge_with_gap() -> None:
    """Number and multiplier remain apart with a gap."""
    board = [
        [2, 0, 0, 0],
        [-2, 0, 0, 0],
        [0, 0, 0, 0],
        [16, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, 0)
    assert new_board[1][0] == 2
    assert new_board[2][0] == -2
    assert new_board[3][0] == 16
    assert delta == 0


def test_step_rejects_non_power_of_two() -> None:
    """Reject tiles that are not powers of two."""
    board = [
        [3, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError, match=r"^invalid tile value: 3$"):
        ak.step(board, 0)


def test_step_rejects_too_large_value() -> None:
    """Reject tiles larger than the maximum allowed."""
    board = [
        [131072, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError, match=r"^invalid tile value: 131072$"):
        ak.step(board, 0)


def test_step_rejects_unknown_negative_multiplier() -> None:
    """Reject negative multipliers other than -1, -2, or -4."""
    board = [
        [-3, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError, match=r"^invalid tile value: -3$"):
        ak.step(board, 0)


def test_step_rejects_one() -> None:
    """Reject the value one, which is not allowed."""
    board = [
        [1, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    with pytest.raises(ValueError, match=r"^invalid tile value: 1$"):
        ak.step(board, 0)
