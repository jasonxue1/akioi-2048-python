// Re-export the published engine crate's Rust API
pub use ak_engine::{Direction, State, init, step};

// Python bindings delegating to the published crate
#[cfg(feature = "python-bindings")]
mod py_api {
    use pyo3::prelude::*;
    use pyo3::types::{PyAny, PyModule};

    /// Create a new 4x4 board with two starting tiles.
    ///
    /// Returns:
    ///     list[list[int]]: Fresh board ready for play.
    #[pyfunction]
    #[must_use]
    pub fn init() -> Vec<Vec<i32>> {
        let board = ak_engine::init();
        board.iter().map(|r| r.to_vec()).collect()
    }

    #[pyfunction]
    /// Apply one move. If the board changes, a new tile appears in a random empty cell.
    ///
    /// Args:
    ///     board: 4x4 board. Positive numbers are normal tiles (2, 4, 8, ...).
    ///            Negative numbers are multipliers: -1=x1, -2=x2, -4=x4
    ///            (absolute value is the multiplier).
    ///     direction: Move direction enum: Direction.{Up,Down,Left,Right}
    ///
    /// Returns:
    ///     tuple[list[list[int]], int, State]: (new_board, delta_score, state)
    ///         where state is State.{Victory, GameOver, Continue}.
    ///
    /// Notes:
    ///     If the board does not change, no tile is spawned and delta_score=0.
    pub fn step(
        board: &Bound<'_, PyAny>,
        direction: &Bound<'_, PyAny>,
    ) -> PyResult<(Vec<Vec<i32>>, i32, Py<PyAny>)> {
        let board4: [[i32; 4]; 4] = board.extract()?;
        let dir = parse_direction(direction)?;
        match ak_engine::step(board4, dir) {
            Ok((next, delta, state)) => {
                let py = board.py();
                let py_state = state_to_py(py, state)?;
                Ok((next.iter().map(|r| r.to_vec()).collect(), delta, py_state))
            }
            Err(msg) => Err(pyo3::exceptions::PyValueError::new_err(msg)),
        }
    }

    fn parse_direction(py_dir: &Bound<'_, PyAny>) -> PyResult<ak_engine::Direction> {
        let name: String = py_dir.getattr("name")?.extract()?;
        match name.as_str() {
            "Down" => Ok(ak_engine::Direction::Down),
            "Right" => Ok(ak_engine::Direction::Right),
            "Up" => Ok(ak_engine::Direction::Up),
            "Left" => Ok(ak_engine::Direction::Left),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "direction must be a Direction enum, got: {name}"
            ))),
        }
    }

    fn state_to_py(py: Python<'_>, state: ak_engine::State) -> PyResult<Py<PyAny>> {
        let pkg = PyModule::import(py, "akioi_2048")?;
        let cls = pkg.getattr("State")?;
        let variant = match state {
            ak_engine::State::Victory => "Victory",
            ak_engine::State::GameOver => "GameOver",
            ak_engine::State::Continue => "Continue",
        };
        Ok(cls.getattr(variant)?.unbind())
    }

    /// Python module for the akioi 2048 engine.
    ///
    /// Exposes:
    /// - init() -> list[list[int]]
    /// - step(board, direction) -> tuple[new_board, delta, State]
    #[pymodule]
    fn akioi_2048(_py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add_function(wrap_pyfunction!(step, module)?)?;
        module.add_function(wrap_pyfunction!(init, module)?)?;
        Ok(())
    }
}
