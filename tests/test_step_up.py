import akioi_2048 as ak


def test_up_number_merges_and_positive_score() -> None:
    board = [
        [4, 0, 0, 0],
        [4, 0, 0, 0],
        [2, 0, 0, 0],
        [2, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, ak.Direction.Up)
    assert new_board[0][0] == 8
    assert new_board[1][0] == 4
    assert delta == 12


def test_up_multiplier_merges_and_negative_score() -> None:
    board = [
        [-2, 0, 0, 0],
        [-2, 0, 0, 0],
        [-1, 0, 0, 0],
        [-1, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, ak.Direction.Up)
    assert new_board[0][0] == -4
    assert new_board[1][0] == -2
    assert delta == -6


def test_up_number_multiplier_merges() -> None:
    board = [
        [-2, 0, 0, 0],
        [2, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, ak.Direction.Up)
    assert new_board[0][0] == 4
    assert delta == 4


def test_up_number_and_multiplier_do_not_merge_without_tiles_above() -> None:
    board = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [-2, 0, 0, 0],
        [2, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, ak.Direction.Up)
    assert new_board[0][0] == -2
    assert new_board[1][0] == 2
    assert delta == 0


def test_up_number_and_multiplier_no_merge_with_gap() -> None:
    board = [
        [16, 0, 0, 0],
        [0, 0, 0, 0],
        [-2, 0, 0, 0],
        [2, 0, 0, 0],
    ]
    new_board, delta, _ = ak.step(board, ak.Direction.Up)
    assert new_board[0][0] == 16
    assert new_board[1][0] == -2
    assert new_board[2][0] == 2
    assert delta == 0
