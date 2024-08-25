from src.game import Game
from src.world import LiveWorld

if __name__ == '__main__':
    world = LiveWorld((256, 256))
    game = Game(world, fps=60, width=1280, height=1280)
    game.run()
