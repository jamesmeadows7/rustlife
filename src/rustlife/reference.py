def step(cells: list[list[bool]]) -> list[list[bool]]:
    """Return the next generation as a new grid."""
    height = len(cells)
    width = len(cells[0])
    return [[_next_state(cells, r, c) for c in range(width)] for r in range(height)]


def _next_state(cells: list[list[bool]], row: int, col: int) -> bool:
    height = len(cells)
    width = len(cells[0])
    alive = cells[row][col]
    count = sum(
        cells[(row + dr) % height][(col + dc) % width]
        for dr in (-1, 0, 1)
        for dc in (-1, 0, 1)
        if (dr, dc) != (0, 0)
    )
    return count == 3 or (alive and count == 2)
