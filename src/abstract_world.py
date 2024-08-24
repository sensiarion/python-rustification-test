import abc
import typing

import pygame

if typing.TYPE_CHECKING:
    from src.game import Game


class AbstractWorld(abc.ABC):
    _game: "Game"
    """Instance of game. should be set in start function. (will be cleared on restarting)"""

    @abc.abstractmethod
    def update(self, delta: float):
        pass

    @abc.abstractmethod
    def render(self, screen: pygame.Surface):
        pass

    @abc.abstractmethod
    def start(self, game: "Game"):
        pass

    @abc.abstractmethod
    def stop(self):
        pass

    @abc.abstractmethod
    def copy(self) -> "AbstractWorld":
        pass
