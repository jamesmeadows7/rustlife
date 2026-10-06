import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
from matplotlib.image import AxesImage

from rustlife import Life


def pattern(*rows: str) -> list[list[bool]]:
    return [[c == "#" for c in row] for row in rows]


def place(life: Life, rows: list[list[bool]], top: int, left: int) -> None:
    for r, cells in enumerate(rows):
        for c, alive in enumerate(cells):
            if alive:
                life.set(top + r, left + c, True)


def update(frame: int, life: Life, image: AxesImage) -> list[AxesImage]:
    life.step()
    image.set_data(life.to_list())
    return [image]


life = Life(40, 40)
place(life, pattern(".#.", "..#", "###"), top=2, left=2)

fig, ax = plt.subplots()
image = ax.imshow(life.to_list(), cmap="binary", vmin=0, vmax=1)
ax.set_xticks([])
ax.set_yticks([])
anim = FuncAnimation(fig, update, fargs=(life, image), interval=50)
plt.show()
