import akioi_2048 as ak


def test_invalid_move_triggers_failure() -> None:
    board = [
        [2, 4, 2, 4],
        [4, 2, 4, 2],
        [2, 4, 2, 4],
        [4, 2, 4, 2],
    ]
    new_board, delta, msg = ak.step(board, ak.Direction.Down)
    assert new_board == board
    assert delta == 0
    assert msg == ak.State.GameOver


def test_invalid_move_no_failure() -> None:
    board = [
        [2, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]
    new_board, delta, msg = ak.step(board, ak.Direction.Up)
    assert new_board == board
    assert delta == 0
    assert msg == ak.State.Continue
