import enum


class CellState(enum.IntEnum):
    dead = 0
    alive = 1


class Cell:
    __slots__ = ('pos_x', 'pos_y', 'state')

    def __init__(self, pos_x, pos_y, state: CellState):
        self.pos_x = pos_x
        self.pos_y = pos_y
        self.state = state

    def __repr__(self):
        return str(self.state.value)
