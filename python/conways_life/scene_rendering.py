import requests

from .cell_shape import CellShape


class SceneRenering:
    shapes_api: str = "http://127.0.0.1:3000/shapes"
    multiple_shapes_api: str = "http://127.0.0.1:3000/multiple_shapes"
    clear_api: str = "http://127.0.0.1:3000/clear"

    @classmethod
    def render(cls, shapes: list[CellShape]):
        shapes_data = {"shapes": [shape.to_api_dict() for shape in shapes]}
        requests.post(cls.multiple_shapes_api, json=shapes_data)
