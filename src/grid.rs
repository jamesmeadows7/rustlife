use std::{error::Error, fmt};

#[derive(Clone, Eq, PartialEq)]
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
            return Err(OutOfBounds { row, col });
        }
        let index = self.index(row, col);
        self.cells[index] = alive;
        Ok(())
    }

    fn live_neighbours(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for row_offset in [self.height - 1, 0, 1] {
            for col_offset in [self.width - 1, 0, 1] {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }
                let neighbour_row = (row + row_offset) % self.height;
                let neighbour_col = (col + col_offset) % self.width;
                if self.cells[self.index(neighbour_row, neighbour_col)] {
                    count += 1;
                }
            }
        }
        count
    }

    #[must_use = "step returns the next generation; it does not modify the grid"]
    pub fn step(&self) -> Self {
        let mut next = Self::new(self.height, self.width);
        for row in 0..self.height {
            for col in 0..self.width {
                let alive = self.cells[self.index(row, col)];
                let n = self.live_neighbours(row, col);
                match (alive, n) {
                    (true, 2 | 3) | (false, 3) => {
                        let index = next.index(row, col);
                        next.cells[index] = true;
                    }
                    _ => {}
                }
            }
        }
        next
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid {}x{}", self.height, self.width)?;
        for row in self.cells.chunks(self.width) {
            for alive in row {
                write!(f, "{}", if *alive { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutOfBounds {
    pub row: usize,
    pub col: usize,
}

impl fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cell ({}, {}) is out of bounds", self.row, self.col)
    }
}

impl Error for OutOfBounds {}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid_from(rows: &[&str]) -> Grid {
        let height = rows.len();
        let width = rows[0].chars().count();
        let mut grid = Grid::new(height, width);
        for (row, line) in rows.iter().enumerate() {
            assert_eq!(line.chars().count(), width);
            for (col, c) in line.chars().enumerate() {
                match c {
                    '#' => grid.set(row, col, true).unwrap(),
                    '.' => {}
                    _ => panic!("unexpected character {c:?} at ({row}, {col})"),
                }
            }
        }
        grid
    }

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
        assert_eq!(grid.set(3, 0, true), Err(OutOfBounds { row: 3, col: 0 }));
        assert_eq!(grid.set(0, 4, true), Err(OutOfBounds { row: 0, col: 4 }));
        assert_eq!(grid, before);
    }

    #[test]
    fn out_of_bounds_message() {
        let err = OutOfBounds { row: 3, col: 0 };
        assert_eq!(err.to_string(), "cell (3, 0) is out of bounds");
    }

    #[test]
    fn lone_cell_has_zero_neighbours() {
        let mut grid = Grid::new(5, 5);
        grid.set(2, 2, true).unwrap();
        assert_eq!(grid.live_neighbours(2, 2), 0);
    }

    #[test]
    fn surrounded_cell_has_eight_neighbours() {
        let grid = grid_from(&[".....", ".###.", ".###.", ".###.", "....."]);
        assert_eq!(grid.live_neighbours(2, 2), 8);
    }

    #[test]
    fn horizontal_wrapping() {
        let mut grid = Grid::new(5, 5);
        grid.set(0, 4, true).unwrap();
        assert_eq!(grid.live_neighbours(0, 0), 1);
    }

    #[test]
    fn diagonal_wrapping() {
        let mut grid = Grid::new(5, 5);
        grid.set(4, 4, true).unwrap();
        assert_eq!(grid.live_neighbours(0, 0), 1);
    }

    #[test]
    fn non_square_grid() {
        let mut grid = Grid::new(3, 5);
        grid.set(0, 4, true).unwrap();
        assert_eq!(grid.live_neighbours(0, 0), 1);
    }

    #[test]
    fn block_is_stable() {
        let grid = grid_from(&["....", ".##.", ".##.", "...."]);
        assert_eq!(grid.step(), grid);
    }

    #[test]
    fn blinker_oscillates() {
        let grid = grid_from(&[".....", ".....", ".###.", ".....", "....."]);
        let expected = grid_from(&[".....", "..#..", "..#..", "..#..", "....."]);
        let next = grid.step();
        assert_eq!(next, expected);
        assert_eq!(next.step(), grid);
    }

    #[test]
    fn glider_moves() {
        let grid = grid_from(&[".#...", "..#..", "###..", ".....", "....."]);
        let expected = grid_from(&[".....", "..#..", "...#.", ".###.", "....."]);
        assert_eq!(grid.step().step().step().step(), expected)
    }

    #[test]
    fn glider_wraps_around() {
        let start = grid_from(&[".#...", "..#..", "###..", ".....", "....."]);
        let mut grid = start.clone();
        for _ in 0..20 {
            grid = grid.step();
        }
        assert_eq!(grid, start);
    }
}
