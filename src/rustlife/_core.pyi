from typing import Final

class Life:
    """A Game of Life grid with wrapping edges"""

    MAX_CELLS: Final[int]
    def __init__(self, height: int, width: int) -> None: ...
    @property
    def height(self) -> int: ...
    @property
    def width(self) -> int: ...
    def get(self, row: int, col: int) -> bool:
        """Return whether the cell is alive. Raises IndexError if out of range"""
    def set(self, row: int, col: int, alive: bool) -> None: ...
