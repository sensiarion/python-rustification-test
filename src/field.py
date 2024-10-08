import random
from typing import Iterable

import live_game
import numpy as np

from src.cell import CellState, Cell

#
# class Field:
#     def __init__(self, size: tuple[int, int] = (64, 48), init_state: np.ndarray[int] = None):
#         def _is_alive_on_start():
#             if random.random() <= 0.1:
#                 return CellState.alive
#             return CellState.dead
#
#         if init_state is None:
#             self._size = size
#             self._field = [[Cell(i, j, _is_alive_on_start()) for j in range(size[1])] for i in range(size[0])]
#         else:
#             self._size = size
#             self._field = init_state
#
#     def iterate(self) -> Iterable[tuple[int, int, Cell]]:
#         for x, row in enumerate(self._field):
#             for y, col in enumerate(row):
#                 cell = self._get(x, y)
#                 yield x, y, cell
#
#     def _get(self, x, y) -> Cell:
#         # noinspection PyTypeChecker
#         return self._field[x % self._size[0]][y % self._size[1]]
#
#     def _neighbors(self, x: int, y: int) -> int:
#         t =  live_game.neighbors(x,y,self._field,self._size)
#         return t
#         # offsets = (
#         #     (-1, -1), (-1, 0), (-1, 1),
#         #     (0, -1), (0, 1),
#         #     (1, -1), (1, 0), (1, 1),
#         # )
#         # alive_counter = 0
#         # for offset in offsets:
#         #
#         #     # if self._get(x + offset[0], y + offset[1]).state == CellState.alive:
#         #     t = live_game.get_from_field(x + offset[0], y + offset[1], self._field, self._size)
#         #     if t.state == CellState.alive:
#         #         alive_counter += 1
#         #
#         # return alive_counter
#
#     def copy(self) -> "Field":
#         return Field(init_state=self._field.copy())