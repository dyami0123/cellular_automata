from dataclasses import dataclass


@dataclass
class CellShape:
    x: float
    y: float
    width: float
    height: float

    color: tuple[float, float, float, float]

    def to_api_dict(self):
        return {
            "shape_id": "Rectangle",
            "points": [
                [self.x, self.y],
                [self.x + self.width, self.y + self.height],
            ],
            "size": 0,
            "appearance": {
                "color": {
                    "r": self.color[0],
                    "g": self.color[1],
                    "b": self.color[2],
                    "a": self.color[3],
                }
            },
        }
