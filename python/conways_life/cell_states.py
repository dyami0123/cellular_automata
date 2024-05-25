from enum import Enum
from random import random


class CellStates(Enum):
    DEAD = 0
    ALIVE = 1

    @classmethod
    def random(cls) -> "CellStates":
        return cls.DEAD if random() < 0.5 else cls.ALIVE
