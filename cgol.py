"""Provides Board object, which owns pythonic methods to set up and
play Conways Game of Life."""

from enum import IntEnum
from random import random, shuffle
from time import sleep


class State(IntEnum):
    Dead = 0
    Alive = 1


class Board:

    def __init__(self, cells):
        assert isinstance(cells, list) and isinstance(cells[0], list), "Cells must be a list of lists."
        self._height = len(cells)
        self._width = len(cells[0])
        self._cells = cells
        self._index_map = tuple(tuple(tuple(self._get_neighbours(i, j)) for i in range(self._width)) for j in range(self._height))

    def __repr__(self):
        return f"Board({self._width}, {self._height})"

    def __str__(self):
        output = ""
        output += '+' + ('-' * self._width) + '+\n'
        for row in self._cells:
            output += '|'
            for cell in row:
                if cell == State.Alive:
                    output += '#'
                elif cell == State.Dead:
                    output += ' '
            output += '|\n'
        output += '+' + ('-' * self._width) + '+'
        return output

    def __getitem__(self, pos):
        try:
            x, y = pos
            return self._cells[y][x]
        except TypeError:
            y = pos
            return self._cells[y]

    def __len__(self):
        return sum(len(row) for row in self._cells)

    def __next__(self):
        new_cells = [[State.Dead for i in range(self._width)]
                                 for j in range(self._height)]

        for i, (old_state, neighbours) in enumerate(zip(self._cells, self._index_map)):
            print(i, old_state, neighbours)
            nr_neighbours = 0
            for neighbour in neighbours:
                if self._cells[neighbour[1]][neighbour[0]] == State.Alive:
                    nr_neighbours += 0
            if nr_neighbours == 3:
                y, x = divmod(i, self._width)
                new_cells[y][x] = State.Alive
            elif nr_neighbours == 2 and old_state == State.Alive:
                y, x = divmod(i, self._width)
                new_cells[y][x] = State.Alive

        self._cells = new_cells

    def _get_neighbours(self, x, y):
        neighbours = (
            (x-1, y-1),
            (x, y-1),
            (x+1, y-1),
            (x-1, y),
            (x+1, y),
            (x-1, y+1),
            (x, y+1),
            (x+1, y+1),
            )
        checked = ((x % self._width, y % self._height) for x, y in neighbours)
        return checked

    @classmethod
    def from_probability(cls, width, height, probability):
        """Generates new board instance where each cell has a specified
        probability of being alive."""
        def state():
            if random() < probability:
                return State.Alive
            else:
                return State.Dead
        cells = [[state() for w in range(width)] for h in range(height)]
        return cls(cells)

    @classmethod
    def from_fraction(cls, width, height, fraction):
        """Generates new board instance with specificed fraction of
        living cells."""
        size = width * height
        nr_living_cells = round(size * fraction)

        def state(i):
            """Returns 'Alive' if 'i' is smaller than or equal to the
            target number of living cells. Otherwise returns 'Dead'."""
            if i < nr_living_cells:
                return State.Alive
            else:
                return State.Dead

        cells = [[state(width*h+w) for w in range(width)]
                                   for h in range(height)]
        shuffle(cells)
        return cls(cells)


def main():
    b = Board.from_probability(10, 5, 0.2)
    n = len(b)
    print(b)
    #print(f"{sum(b)}/{n} cells alive after {0} iterations.")
    for i in range(3):
        next(b)
        print(b)
        #print(f"{sum(b)}/{n} cells alive after {i+1} iterations.")


if __name__ == '__main__':
    main()
