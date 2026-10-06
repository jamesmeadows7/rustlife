import pytest

from rustlife import Life


def pattern(*rows: str) -> list[list[bool]]:
    return [[c == "#" for c in row] for row in rows]


@pytest.fixture
def life() -> Life:
    return Life(3, 4)


@pytest.fixture
def glider() -> list[list[bool]]:
    return pattern(".#...", "..#..", "###..", ".....", ".....")


class TestConstruction:
    def test_dimensions(self, life: Life):
        assert life.height == 3
        assert life.width == 4

    def test_new_grid_is_all_dead(self, life: Life):
        assert life.to_list() == [[False] * 4] * 3

    @pytest.mark.parametrize("height, width", [(0, 5), (5, 0), (2**40, 2**40)])
    def test_invalid_dimensions_raise(self, height, width):
        with pytest.raises(ValueError):
            Life(height, width)

    def test_negative_dimensions_raise_overflow(self):
        with pytest.raises(OverflowError):
            Life(-3, -4)

    def test_max_cells_boundary(self):
        life = Life(1, Life.MAX_CELLS)
        assert life.width == Life.MAX_CELLS
        with pytest.raises(ValueError, match="maximum"):
            Life(1, Life.MAX_CELLS + 1)


class TestGetSet:
    def test_set_then_get(self, life: Life):
        life.set(1, 2, True)
        assert life.get(1, 2) is True

    def test_set_can_kill(self, life: Life):
        life.set(0, 0, True)
        life.set(0, 0, False)
        assert life.get(0, 0) is False

    @pytest.mark.parametrize("row, col", [(3, 0), (0, 4)])
    def test_get_out_of_bounds_raises(self, life, row, col):
        with pytest.raises(IndexError, match=rf"\({row}, {col}\)"):
            life.get(row, col)

    @pytest.mark.parametrize("row, col", [(3, 0), (0, 4)])
    def test_set_out_of_bounds_raises(self, life, row, col):
        with pytest.raises(IndexError, match=rf"\({row}, {col}\)"):
            life.set(row, col, True)


class TestStep:
    def test_step_returns_none(self, life: Life):
        assert life.step() is None

    def test_blinker_oscillates(self):
        life = Life.from_list(pattern(".....", ".....", ".###.", ".....", "....."))
        life.step()
        assert life.to_list() == pattern(".....", "..#..", "..#..", "..#..", ".....")
        life.step()
        assert life.to_list() == pattern(".....", ".....", ".###.", ".....", ".....")

    def test_step_n_equals_repeated_steps(self, glider):
        life1 = Life.from_list(glider)
        life1.step(3)
        life2 = Life.from_list(glider)
        life2.step()
        life2.step()
        life2.step()
        assert life1.to_list() == life2.to_list()

    def test_step_zero_is_noop(self, glider):
        life = Life.from_list(glider)
        life.step(0)
        assert life.to_list() == glider


class TestConversions:
    def test_to_list_from_list_round_trip(self, glider):
        assert Life.from_list(glider).to_list() == glider

    def test_from_list_ragged_rows_raise(self):
        with pytest.raises(ValueError, match="row 1"):
            Life.from_list([[True, False], [True]])

    @pytest.mark.parametrize("rows", [[], [[]]])
    def test_from_list_empty_raises(self, rows):
        with pytest.raises(ValueError):
            Life.from_list(rows)

    def test_from_list_non_bool_raises(self):
        with pytest.raises(TypeError, match="bool"):
            Life.from_list([[1, 0]])  # ty: ignore[invalid-argument-type]


class TestDisplay:
    def test_str_renders_grid(self, life: Life):
        life.set(1, 2, True)
        assert str(life) == "....\n..#.\n...."

    def test_repr(self, life):
        assert repr(life) == "<Life 3x4>"
