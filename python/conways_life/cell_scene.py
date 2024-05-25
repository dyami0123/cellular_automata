from dataclasses import dataclass

import numpy as np

from .cell_shape import CellShape
from .cell_states import CellStates


@dataclass
class CellScene:
    data: np.ndarray

    x0: float
    y0: float

    cell_width: float = 10
    cell_height: float = 10

    cell_spacing_x: float = 1
    cell_spacing_y: float = 1

    cell_color: tuple[float, float, float, float] = (1, 1, 1, 1)

    def to_shapes(self) -> list[CellShape]:
        shapes = []

        for i in range(self.data.shape[0]):
            for j in range(self.data.shape[1]):
                if self.data[i, j] == CellStates.DEAD.value:
                    continue

                x = self.x0 + j * (self.cell_width + self.cell_spacing_x)
                y = self.y0 + i * (self.cell_height + self.cell_spacing_y)

                shape = CellShape(
                    x=x,
                    y=y,
                    width=self.cell_width,
                    height=self.cell_height,
                    color=self.cell_color,
                )

                shapes.append(shape)
        return shapes

    @classmethod
    def random(cls, n_rows: int, n_cols: int) -> "CellScene":
        data = np.random.choice(
            [CellStates.DEAD.value, CellStates.ALIVE.value], size=(n_rows, n_cols)
        )
        return cls(data=data, x0=10, y0=10)

    def __repr__(self) -> str:
        return f"CellScene(data={self.data})"
