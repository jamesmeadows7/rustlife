#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        let ncells = height * width;
        Self {
            width,
            height,
            cells: vec![false; ncells],
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.height && col < self.width
    }

    pub fn get(&self, row: usize, col: usize) -> Option<bool> {
        self.in_bounds(row, col)
            .then(|| self.cells[self.index(row, col)])
    }

    pub fn set(&mut self, row: usize, col: usize, alive: bool) -> Result<(), OutOfBounds> {
        if !self.in_bounds(row, col) {
            return Err(OutOfBounds);
        }
        let index = self.index(row, col);
        self.cells[index] = alive;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutOfBounds;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid_has_correct_dimensions() {
        let grid = Grid::new(3, 4);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.width(), 4);
    }

    #[test]
    fn new_grid_is_all_dead() {
        let grid = Grid::new(3, 4);
        for row in 0..grid.height() {
            for col in 0..grid.width() {
                assert_eq!(grid.get(row, col), Some(false));
            }
        }
    }

    #[test]
    fn get_out_of_bounds_returns_none() {
        let grid = Grid::new(3, 4);
        assert_eq!(grid.get(3, 0), None);
        assert_eq!(grid.get(0, 4), None);
    }

    #[test]
    fn check_last_valid_cell() {
        let grid = Grid::new(3, 4);
        assert_eq!(grid.get(2, 3), Some(false));
    }

    #[test]
    fn set_then_get() {
        let mut grid = Grid::new(3, 4);
        grid.set(1, 2, true).unwrap();
        assert_eq!(grid.get(1, 2), Some(true));
    }

    #[test]
    fn set_can_kill() {
        let mut grid = Grid::new(3, 4);
        grid.set(0, 0, true).unwrap();
        grid.set(0, 0, false).unwrap();
        assert_eq!(grid.get(0, 0), Some(false));
    }

    #[test]
    fn set_out_of_bounds_error() {
        let mut grid = Grid::new(3, 4);
        let before = grid.clone();
        assert_eq!(grid.set(3, 0, true), Err(OutOfBounds));
        assert_eq!(grid.set(0, 4, true), Err(OutOfBounds));
        assert_eq!(grid, before);
    }
}
