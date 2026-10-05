from typing import Final

class Life:
    """A Game of Life grid with wrapping edges."""

    MAX_CELLS: Final[int]
    """Maximum number of cells (height x width)."""
    def __init__(self, height: int, width: int) -> None:
        """Create an all-dead grid, validating the dimensions."""
    @property
    def height(self) -> int:
        """Number of rows."""
    @property
    def width(self) -> int:
        """Number of columns."""
    def set(self, row: int, col: int, alive: bool) -> None:
        """Set the cell at (row, col) to alive or dead. Raises IndexError if (row, col) is outside of grid."""
    def get(self, row: int, col: int) -> bool:
        """Return True if the cell at (row, col) is alive. Raises IndexError if (row, col) is outside of grid."""
    def step(self, n: int = 1) -> None:
        """Advance the simulation by n generations, in place."""
