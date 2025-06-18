// src/lib.rs
//! akioi / oi-2048 backend (pure logic) + PyO3 bridge.
//!
//! Python 使用：
//! ```python
//! import akioi_2048 as ak
//! new_bd, delta, msg = ak.step(board, 2)   # 2 = left
//! ```

use pyo3::prelude::*;
use pyo3::types::PyAny;

use rand::{prelude::*, seq::SliceRandom};

/// 4×4 棋盘类型
pub type Board = [[i32; 4]; 4];

/// 内部方向枚举
#[derive(Clone, Copy)]
enum Action {
    Up,
    Down,
    Left,
    Right,
}

/// ---------- Python 公开函数 -------------------------------------------------

/// `step(board, dir)`
///
/// * `board` — 4×4 `list[list[int]]`  
/// * `dir`   — 0=Up 1=Down 2=Left 3=Right  
///
/// 返回 `(new_board, delta_score, msg)`  
/// * `msg = 1` 胜利 / `-1` 失败 / `0` 继续
#[pyfunction]
fn step(py_board: &PyAny, dir: u8) -> PyResult<(Vec<Vec<i32>>, i32, i8)> {
    // ① Python → Rust Board
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

    // ② dir → Action
    let action = match dir {
        0 => Action::Up,
        1 => Action::Down,
        2 => Action::Left,
        3 => Action::Right,
        _ => return Err(pyo3::exceptions::PyValueError::new_err("dir must be 0-3")),
    };

    let mut rng = rand::thread_rng();

    // ③ 执行一步
    let (mut next, delta, victory) = single_step(&board, action);

    let moved = next != board;
    if moved {
        spawn_tile(&mut next, &mut rng); // 规则：有效移动后随机生成一块
    }

    // ④ 判断失败（四方向都无法动）
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

/// ---------- 纯逻辑 ---------------------------------------------------------

/// 返回 `(new_board, delta_score, victory?)`（不生成随机砖）
fn single_step(board: &Board, action: Action) -> (Board, i32, bool) {
    let rot = match action {
        Action::Up => 0,
        Action::Down => 2,
        Action::Left => 3,
        Action::Right => 1,
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

/// 旋转棋盘 90°×k（顺时针）
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

/// 处理单列，下落合并；返回 `(新列, 得分增量)`
fn slide_column(col: [i32; 4]) -> ([i32; 4], i32) {
    let mut buf = Vec::with_capacity(4);
    let mut score = 0;
    let mut i: i32 = 3;
    while i >= 0 {
        if col[i as usize] == 0 {
            if i == 0 {
                break;
            }
            i -= 1;
            continue;
        }
        // 在上方找最近非零
        let mut j = i - 1;
        while j >= 0 && col[j as usize] == 0 {
            j -= 1;
        }
        if j >= 0 {
            if let Some((merged, add)) = try_merge(
                col[i as usize],
                col[j as usize],
                i == j + 1,
                &col[(i as usize + 1)..],
            ) {
                buf.push(merged);
                score += add;
                i = if j > 0 { j - 1 } else { 0 };
                continue;
            }
        }
        buf.push(col[i as usize]);
        if i == 0 {
            break;
        }
        i -= 1;
    }
    while buf.len() < 4 {
        buf.push(0);
    }
    buf.reverse();
    ([buf[0], buf[1], buf[2], buf[3]], score)
}

/// 判定并执行合并
fn try_merge(a: i32, b: i32, adjacent: bool, below: &[i32]) -> Option<(i32, i32)> {
    // 数值 + 数值
    if a > 0 && b > 0 && a == b && a < 65_536 {
        return Some((a + b, a + b));
    }
    // 倍增 + 倍增
    if a < 0 && b < 0 && a == b && a > -4 {
        return Some((a * 2, a * 2));
    }
    // 数值 + 倍增
    if a * b < 0 && adjacent && below.iter().all(|&v| v == 0) {
        let num = if a > 0 { a } else { b };
        let mul = if a < 0 { a } else { b };
        let v = num * mul.abs();
        return Some((v, v));
    }
    None
}

/// 随机在空格生成一块新砖（权重同网页规则）
fn spawn_tile<R: Rng>(board: &mut Board, rng: &mut R) {
    // ① 收集空格坐标（不再用闭包，避免 move）
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

    // ② 随机挑选位置
    let &(r, c) = empties.choose(rng).unwrap();

    // ③ 按权重生成方块
    let p: f64 = rng.gen();
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

/// 注册 Python 模块
#[pymodule]
fn akioi_2048(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(step, m)?)?;
    Ok(())
}
