import akioi_2048 as ak

BOARD_SIZE = 4
VALID_STATES = {ak.State.GameOver, ak.State.Continue, ak.State.Victory}


def test_step_smoke() -> None:
    board = ak.init()
    new_board, delta, msg = ak.step(board, ak.Direction.Down)
    assert len(new_board) == BOARD_SIZE
    assert all(len(r) == BOARD_SIZE for r in new_board)
    assert isinstance(delta, int)
    assert msg in VALID_STATES
