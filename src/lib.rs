use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::prelude::*;

mod grid;
use grid::{Grid, OutOfBounds};

/// A Game of Life grid with wrapping edges.
#[pyclass(module = "rustlife")]
struct Life {
    grid: Grid,
}

#[pymethods]
impl Life {
    /// Maximum number of cells (height x width).
    #[classattr]
    const MAX_CELLS: usize = 100_000_000;

    /// Create an all-dead grid, validating the dimensions.
    #[new]
    fn new(height: usize, width: usize) -> PyResult<Self> {
        if height == 0 || width == 0 {
            return Err(PyValueError::new_err(format!(
                "grid dimensions must be positive, got {height}x{width}"
            )));
        }
        match height.checked_mul(width) {
            Some(n) if n <= Self::MAX_CELLS => {}
            _ => {
                return Err(PyValueError::new_err(format!(
                    "grid of {height}x{width} exceeds the maximum of {} cells",
                    Self::MAX_CELLS
                )));
            }
        }
        Ok(Self {
            grid: Grid::new(height, width),
        })
    }

    /// Number of rows.
    #[getter]
    fn height(&self) -> usize {
        self.grid.height()
    }

    /// Number of columns.
    #[getter]
    fn width(&self) -> usize {
        self.grid.width()
    }

    /// Set the cell at (row, col) to alive or dead.
    fn set(&mut self, row: usize, col: usize, alive: bool) -> PyResult<()> {
        Ok(self.grid.set(row, col, alive)?)
    }

    /// Return True if the cell at (row, col) is alive.
    fn get(&self, row: usize, col: usize) -> PyResult<bool> {
        Ok(self.grid.get(row, col).ok_or(OutOfBounds { row, col })?)
    }

    /// Advance the simulation by n generations, in place.
    #[pyo3(signature = (n = 1))]
    fn step(&mut self, n: usize) {
        for _ in 0..n {
            self.grid = self.grid.step();
        }
    }

    fn __str__(&self) -> String {
        self.grid.to_string()
    }

    fn __repr__(&self) -> String {
        format!("<Life {}x{}>", self.grid.height(), self.grid.width())
    }

    /// Return the grid as a list of rows, each a list of bools (True = alive).
    fn to_list(&self) -> Vec<Vec<bool>> {
        self.grid.to_rows()
    }

    /// Create a grid from a list of rows, each a list of bools (True = alive). Raises ValueError if the rows differ in length or the size is invalid.
    #[staticmethod]
    fn from_list(rows: Vec<Vec<bool>>) -> PyResult<Self> {
        let height = rows.len();
        let width = rows.first().map_or(0, |row| row.len());
        let mut life = Self::new(height, width)?;
        for (row, cells) in rows.iter().enumerate() {
            if cells.len() != width {
                return Err(PyValueError::new_err(format!(
                    "row {row} has length {}, expected {width}",
                    cells.len()
                )));
            }
            for (col, &alive) in cells.iter().enumerate() {
                if alive {
                    life.grid.set(row, col, true)?;
                }
            }
        }
        Ok(life)
    }
}

impl From<OutOfBounds> for PyErr {
    fn from(err: OutOfBounds) -> Self {
        PyIndexError::new_err(err.to_string())
    }
}

#[pymodule]
mod _core {
    #[pymodule_export]
    use super::Life;
}
