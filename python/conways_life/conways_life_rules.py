import numpy as np

from .cell_scene import CellScene
from .cell_states import CellStates


class ConwayRules:

    @classmethod
    def evolve_scene(cls, scene: CellScene) -> CellScene:
        new_data = cls.apply_rules(scene.data)
        return CellScene(data=new_data, x0=scene.x0, y0=scene.y0)

    @staticmethod
    def apply_rules(data: np.ndarray) -> np.ndarray:
        padded_data = np.pad(
            data, pad_width=1, mode="constant", constant_values=CellStates.DEAD.value
        )
        new_data = data.copy()

        for i in range(1, padded_data.shape[0] - 1):
            for j in range(1, padded_data.shape[1] - 1):
                num_neighbours = (
                    np.sum(padded_data[i - 1 : i + 2, j - 1 : j + 2])
                    - padded_data[i, j]
                )
                new_data[i - 1, j - 1] = ConwayRules.apply_rule(
                    padded_data[i, j], num_neighbours
                )

        return new_data

    @staticmethod
    def apply_rule(cell: int, num_neighbours: int) -> int:
        if cell == CellStates.ALIVE.value:
            return (
                CellStates.ALIVE.value
                if 2 <= num_neighbours <= 3
                else CellStates.DEAD.value
            )
        return CellStates.ALIVE.value if num_neighbours == 3 else CellStates.DEAD.value
