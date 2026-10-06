import random

import pytest

from rustlife import Life
from rustlife.reference import step


def random_grid(
    rng: random.Random, height: int, width: int, density: float = 0.4
) -> list[list[bool]]:
    return [[rng.random() < density for _ in range(width)] for _ in range(height)]


class TestReference:
    @pytest.mark.parametrize("seed", range(50))
    def test_matches_reference(self, seed: int):
        rng = random.Random(seed)
        height = rng.randint(Life.MIN_SIZE, 20)
        width = rng.randint(Life.MIN_SIZE, 20)
        cells = random_grid(rng, height, width)
        life = Life.from_list(cells)
        for i in range(20):
            cells = step(cells)
            life.step()
            assert life.to_list() == cells, (
                f"{height}x{width} grid: mismatch at step {i + 1}"
            )
