use pyo3::prelude::*;
use pyo3::types::PyAny;

use rand::prelude::IndexedRandom;
use rand::{Rng, rng};

/// 4×4 board grid type
pub type Board = [[i32; 4]; 4];

/// Internal move direction enum
#[derive(Clone, Copy)]
enum Action {
    Up,
    Down,
    Left,
    Right,
}

/// Apply one move; if the board changes a new tile is spawned at random.
///
/// :param list[list[int]] board:
///     4×4 board matrix.
///     * **Positive values** → Normal value tiles (2, 4, 8, …)
///     * **Negative values** → Multiplier tiles -1=×1, -2=×2, -4=×4; absolute value is the multiplier.
///
/// :param int dir:
///     Move direction:
///     * `0` = **Down**  ↓
///     * `1` = **Right** →
///     * `2` = **Up**    ↑
///     * `3` = **Left**  ←
///
/// :returns: *(new_board, delta_score, msg)*
///     * **new_board** `list[list[int]]` Board after the move
///     * **delta_score** `int` Score gained or lost from merges this move
///     * **msg** `int` Status flag
///         * `1`  → A `65536` tile was created → **Victory**
///         * `-1` → No possible moves in any direction → **Game Over**
///         * `0`  → Continue playing
///
/// :note:
///     If the move is invalid (board unchanged),
///     **no new tile is generated**, `delta_score = 0`, and `msg = 0`.
#[pyfunction]
fn step(py_board: &Bound<'_, PyAny>, direction: u8) -> PyResult<(Vec<Vec<i32>>, i32, i8)> {
    // ① Convert Python list into a Rust board
    let raw: Vec<Vec<i32>> = py_board.extract()?;
    if raw.len() != 4 || raw.iter().any(|r| r.len() != 4) {
        return Err(pyo3::exceptions::PyValueError::new_err("board must be 4×4"));
    }
    let mut board: Board = [[0; 4]; 4];
    for (r, row) in raw.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            board[r][c] = v;
        }
    }

    // ② Map `direction` to `Action`
    let action = match direction {
        0 => Action::Down,
        1 => Action::Right,
        2 => Action::Up,
        3 => Action::Left,
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "direction must be 0-3",
            ));
        }
    };

    let mut rng = rng();

    // ③ Perform one logical step
    let (mut next, delta, victory) = single_step(&board, action);

    let moved = next != board;
    if moved {
        spawn_tile(&mut next, &mut rng); // rule: spawn a tile after a valid move
    }

    // ④ Check failure (no moves in any direction)
    let dead = !moved && (0..4).all(|d| single_step(&next, idx_to_action(d)).0 == next);

    let msg = if victory {
        1
    } else if dead {
        -1
    } else {
        0
    };

    Ok((next.iter().map(|r| r.to_vec()).collect(), delta, msg))
}

/// Initialize a new board with two tiles
///
/// :returns: *new_board*
///     * **new_board** `list[list[int]]` A fresh board
#[pyfunction]
fn init() -> PyResult<Vec<Vec<i32>>> {
    let mut rng = rng();
    let mut board: Board = [[0; 4]; 4];
    spawn_tile(&mut board, &mut rng);
    spawn_tile(&mut board, &mut rng);

    Ok(board.iter().map(|r| r.to_vec()).collect())
}

/// ---------- Pure logic ---------------------------------------------------------

/// Return `(new_board, delta_score, victory?)` (no random tile spawn)
fn single_step(board: &Board, action: Action) -> (Board, i32, bool) {
    let rot = match action {
        Action::Down => 0,  // ↓
        Action::Up => 2,    // ↑ rotate 180°
        Action::Left => 3,  // ← rotate -90°
        Action::Right => 1, // → rotate +90°
    };
    let mut work = rotate(*board, rot);

    let mut delta = 0;
    for c in 0..4 {
        let (col, add) = slide_column([work[0][c], work[1][c], work[2][c], work[3][c]]);
        delta += add;
        for r in 0..4 {
            work[r][c] = col[r];
        }
    }
    let next = rotate(work, (4 - rot) % 4);
    let victory = next.iter().flatten().any(|&v| v == 65_536);
    (next, delta, victory)
}

fn idx_to_action(i: usize) -> Action {
    [Action::Up, Action::Down, Action::Left, Action::Right][i]
}

/// Rotate board 90°×k clockwise
fn rotate(b: Board, k: usize) -> Board {
    let mut r = [[0; 4]; 4];
    match k % 4 {
        0 => b,
        1 => {
            for i in 0..4 {
                for j in 0..4 {
                    r[j][3 - i] = b[i][j];
                }
            }
            r
        }
        2 => {
            for i in 0..4 {
                for j in 0..4 {
                    r[3 - i][3 - j] = b[i][j];
                }
            }
            r
        }
        3 => {
            for i in 0..4 {
                for j in 0..4 {
                    r[3 - j][i] = b[i][j];
                }
            }
            r
        }
        _ => r,
    }
}

/// Process one column: scan upward, merge, and drop tiles.
/// Return `(new_column, score_delta)`
///
/// * Scan pointer `r` from 3 down to 0.
/// * Write pointer `w` from 3 down to 0 (always filling bottom up).
fn slide_column(col: [i32; 4]) -> ([i32; 4], i32) {
    let mut out = [0i32; 4];
    let mut w: i32 = 3; // write position (bottom to top)
    let mut score = 0;
    let mut r: i32 = 3; // read pointer (bottom to top)

    while r >= 0 {
        // skip empty cells
        if col[r as usize] == 0 {
            r -= 1;
            continue;
        }

        // find first non-zero above
        let mut s = r - 1;
        while s >= 0 && col[s as usize] == 0 {
            s -= 1;
        }

        // try merging r and s
        let merged = if s >= 0 {
            let below_slice = &col[(r as usize + 1)..4]; // slice is empty if r=3
            try_merge(col[r as usize], col[s as usize], r == s + 1, below_slice)
        } else {
            None
        };

        match merged {
            Some((tile, add)) => {
                out[w as usize] = tile;
                score += add;
                w -= 1;
                r = s - 1; // skip the merged tile
            }
            None => {
                out[w as usize] = col[r as usize];
                w -= 1;
                r -= 1;
            }
        }
    }

    (out, score)
}

/// Determine and perform a merge
fn try_merge(a: i32, b: i32, adjacent: bool, below: &[i32]) -> Option<(i32, i32)> {
    // numeric + numeric
    if a > 0 && b > 0 && a == b && a < 65_536 {
        return Some((a + b, a + b));
    }
    // multiplier + multiplier
    if a < 0 && b < 0 && a == b && a > -4 {
        return Some((a * 2, a * 2));
    }
    // numeric + multiplier
    if a * b < 0 && adjacent && (below.is_empty() || below.iter().all(|&v| v != 0)) {
        let num = if a > 0 { a } else { b };
        let mul = if a < 0 { a } else { b };
        let v = num * mul.abs();
        return Some((v, v));
    }
    None
}

/// Spawn a random tile on an empty cell (same probabilities as the web version)
fn spawn_tile<R: Rng>(board: &mut Board, rng: &mut R) {
    // ① Gather empty coordinates (avoid closure to skip move)
    let mut empties = Vec::new();
    for r in 0..4 {
        for c in 0..4 {
            if board[r][c] == 0 {
                empties.push((r, c));
            }
        }
    }
    if empties.is_empty() {
        return;
    }

    // ② Pick a random position
    let &(r, c) = empties.choose(rng).unwrap();

    // ③ Generate a tile using weighted probabilities
    let p: f64 = rng.random();
    board[r][c] = if p < 0.783 {
        2
    } else if p < 0.861 {
        4
    } else if p < 0.9728 {
        -1 // ×1
    } else {
        -2 // ×2
    };
}

#[pymodule]
fn akioi_2048(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(step, &m)?)?;
    m.add_function(wrap_pyfunction!(init, &m)?)?;
    Ok(())
}
