import typing
from typing import Iterable

import live_game
import pygame.draw

from src.abstract_world import AbstractWorld
from src.cell import CellState, Cell

# from src.field import Field

if typing.TYPE_CHECKING:
    from src.game import Game


class LiveWorld(AbstractWorld):

    def __init__(self, size: tuple[int, int] = (64, 48)):

        self._field = live_game.Field(size)
        self._size = size

        self._game: "Game|None" = None
        self._background_prerender: pygame.Surface | None = None

    def copy(self) -> "AbstractWorld":
        return LiveWorld(size=self._size)

    def iterate(self) -> Iterable[tuple[int, int, Cell]]:
        return self._field.iterate()

    def update(self, delta: float):
        # live_game.update(self._field._field, self._size)
        self._field.update()

    def render(self, screen: pygame.Surface):
        live_game.render(
            screen,
            self._background_prerender,
            pygame.draw.ellipse,
            self._field,
            self._size,
            (self._game._width, self._game._height)
        )
        # full_cell_size = (self._game._height // self._size[0], self._game._width // self._size[1])
        # border_size = (2, 2)
        # cell_size = full_cell_size[0] - border_size[0], full_cell_size[1] - border_size[1]
        #
        # screen.blit(self._background_prerender, (0, 0))
        #
        # for x, y, cell_state in self.iterate():
        #     inner_rect = (
        #         x * cell_size[0] + (x * border_size[0]), y * cell_size[1] + (y * border_size[1]),
        #         cell_size[0], cell_size[1]
        #     )
        #
            # pygame.draw.rect(screen, (255, 255, 255), rect)
        #     if cell_state == CellState.alive:
        #         pygame.draw.ellipse(screen, (255, 255, 255), inner_rect)

    def start(self, game: "Game"):
        self._game = game

        background = pygame.Surface((game._width, game._height))

        height = self._game._height
        width = self._game._width
        full_cell_size = (self._game._height // self._size[0], self._game._width // self._size[1])

        for i in range(1, self._size[0]):
            pygame.draw.line(background, (255, 255, 255), (0, i * full_cell_size[0]), (width, i * full_cell_size[0],))

        for i in range(1, self._size[1]):
            pygame.draw.line(
                background,
                (255, 255, 255),
                (i * full_cell_size[1], 0),
                (i * full_cell_size[1], height)
            )

        self._background_prerender = background

    def stop(self):
        pass
