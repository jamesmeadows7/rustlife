import random
from collections.abc import Callable
from time import perf_counter

from rustlife import Life
from rustlife.reference import step

SEED = 42
SIZE = 200
STEPS = 100
REPEATS = 5

type Grid = list[list[bool]]
type Timer = Callable[[Grid, int], float]


def random_grid(
    rng: random.Random, height: int, width: int, density: float = 0.4
) -> Grid:
    return [[rng.random() < density for _ in range(width)] for _ in range(height)]


def time_python(cells: Grid, steps: int) -> float:
    start = perf_counter()
    for _ in range(steps):
        cells = step(cells)
    return perf_counter() - start


def time_rust(cells: Grid, steps: int) -> float:
    life = Life.from_list(cells)  # setup, not timed
    start = perf_counter()
    life.step(steps)
    return perf_counter() - start


def best_of(timer: Timer, cells: Grid, steps: int) -> float:
    return min(timer(cells, steps) for _ in range(REPEATS))


def main() -> None:
    rng = random.Random(SEED)

    cells = random_grid(rng, SIZE, SIZE)
    cell_steps = SIZE * SIZE * STEPS
    python = best_of(time_python, cells, STEPS)
    rust = best_of(time_rust, cells, STEPS)

    print(f"{SIZE}x{SIZE} grid, {STEPS} steps, best of {REPEATS}\n")
    print(f"{'':<8}{'time (s)':>10}{'ns/cell-step':>15}{'speed-up':>10}")
    for name, seconds in [("python", python), ("rust", rust)]:
        ns = seconds / cell_steps * 1e9
        print(f"{name:<8}{seconds:>10.4f}{ns:>15.1f}{python / seconds:>9.0f}x")


if __name__ == "__main__":
    main()
