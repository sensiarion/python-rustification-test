from collections import defaultdict
from typing import Callable

import pygame.display
import pygame.event
import pygame.font
import pygame.time

from src.abstract_world import AbstractWorld


class Game:
    def __init__(
            self,
            world: AbstractWorld,
            fps: int = 60,
            width: int = 640,
            height: int = 480,
    ):
        pygame.font.init()

        self._height = height
        self._width = width
        self._fps = fps
        self._screen = pygame.display.set_mode((width, height))
        self._timer = pygame.time.Clock()
        self._world = world

        self.font = pygame.font.SysFont('Arial', 18)

        self._handlers: dict[int, list[Callable[[pygame.event.Event], None]]] = defaultdict(list)

    def register_handler(self, event_type: int, handler: Callable[[pygame.event.Event], None]):
        self._handlers[event_type].append(handler)

    # def need_render(self, frame: int) -> bool:
    #     return (frame % self._updates_per_frame) == 0

    def _run(self):
        screen = self._screen
        fps = self._fps
        delta = (1 / fps) / 1000
        iteration = 0

        self._world.start(self)

        running = True
        while running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    print('Exiting...')
                    running = False
                    break
                if event.type == pygame.KEYDOWN and event.key == pygame.K_r:
                    print('Restarting world')
                    new_world = self._world.copy()
                    del self._world._game
                    del self._world
                    self._world = new_world
                    self._world.start(self)
                    print('World restarted')
                if specific_handlers := self._handlers.get(event.type):
                    for handler in specific_handlers:
                        handler(event)

            self.update(delta)
            # if self.need_render(iteration):
            screen.fill((0, 0, 0))
            self.render(screen)

            # fps counter
            update_rate = str(round(1000 / delta, 1))
            print(update_rate)
            update_window_offset = 60
            screen.blit(
                self.font.render(update_rate, 1, (255, 0, 0), (0, 0, 0)),
                (self._width - update_window_offset, 0, update_window_offset, update_window_offset)
            )

            delta = self._timer.tick(fps)
            pygame.display.flip()

            iteration = (iteration + 1) % fps

    def run(self):
        try:
            self._run()
        finally:
            self._world.stop()

    def update(self, delta: float):
        self._world.update(delta)

    def render(self, screen: pygame.Surface):
        self._world.render(screen)
