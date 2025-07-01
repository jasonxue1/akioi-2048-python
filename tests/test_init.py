import sys
import akioi_2048 as ak
from printer import print_table
from tqdm import tqdm

ALLOWED = {-2, -1, 2, 4}


def flatten(board):
    """Return a 1-D list of 16 cells, whether `board` is flat or nested."""
    return (
        [c for row in board for c in row]
        if any(isinstance(r, list) for r in board)
        else board
    )


def is_init_board(board) -> bool:
    flat = flatten(board)
    if len(flat) != 16:
        return False
    non_zero = [x for x in flat if x]
    return len(non_zero) == 2 and all(x in ALLOWED for x in non_zero)


def main(trials: int = 1_000_000) -> None:
    print()
    for i in tqdm(range(1, trials + 1)):
        board = ak.init()
        if not is_init_board(board):
            print(f"✗ failed at trial {i}", file=sys.stderr)
            print_table(board)
            sys.exit(1)
    print(f"✓ {trials} trials passed")


if __name__ == "__main__":
    main()
