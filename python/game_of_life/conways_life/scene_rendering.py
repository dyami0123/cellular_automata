import requests

from .cell_shape import CellShape


class SceneRenering:

    shapes_api: str = "http://127.0.0.1:3000/shapes"
    multiple_shapes_api: str = "http://127.0.0.1:3000/multiple_shapes"
    clear_api: str = "http://127.0.0.1:3000/clear"

    # @classmethod
    # async def render_shapes(cls, shapes: list[CellShape]):
    #     async with httpx.AsyncClient() as client:
    #         start_time = time.time()

    #         # Clear existing shapes
    #         clear_data = {"clear": True}
    #         clear_start = time.time()
    #         await client.post(cls.clear_api, json=clear_data)
    #         clear_end = time.time()
    #         print(f"Clearing shapes took {clear_end - clear_start} seconds")

    #         # Post new shapes
    #         tasks = []
    #         for shape in shapes:
    #             tasks.append(client.post(cls.shapes_api, json=shape.to_api_dict()))

    #         shapes_start = time.time()
    #         await asyncio.gather(*tasks)
    #         shapes_end = time.time()
    #         print(f"Posting shapes took {shapes_end - shapes_start} seconds")

    #         total_time = time.time() - start_time
    #         print(f"Total render_shapes execution time: {total_time} seconds")

    @classmethod
    def render(cls, shapes: list[CellShape]):
        # requests.post(cls.clear_api, json={"clear": True})

        shapes_data = {"shapes": [shape.to_api_dict() for shape in shapes]}

        requests.post(cls.multiple_shapes_api, json=shapes_data)
