from os import system
from time import sleep
from random import random

ALIVE = 1
DEAD = 0

WIDTH = 100
HEIGHT = 40
SIZE = WIDTH * HEIGHT


def count_live_neighbours(index, board):
    """Returns the number of neighbouring cells which are alive."""
    nn = index - WIDTH
    nw = nn - 1
    ne = nn + 1
    ww = index - 1
    ee = index + 1
    ss = index + WIDTH
    sw = ss - 1
    se = ss + 1
    sum_ = 0
    for i in [nw, nn, ne, ww, ee, sw, ss, se]:
        j = i % (SIZE)
        if board[j]:
            sum_ += 1
    return sum_


def update_board(board, neighbors):
    new_board = []
    for j, cell in enumerate(board):
        if neighbors[j] == 3:
            new_board.append(ALIVE)
        elif neighbors[j] == 2 and cell == ALIVE:
            new_board.append(ALIVE)
        else:
            new_board.append(DEAD)
    return new_board


def print_board(board):
    output = []
    for i, state in enumerate(board):

        if state == ALIVE:
            output.append('#')
        elif state == DEAD:
            output.append(' ')
        else:
            raise ValueError(f"Undefined state: {state}")

        if i % WIDTH == 0:
            output.append('\n')

    print(''.join(output))


def main():

    board = [1 if (random() < 1/4) else 0 for _ in range(SIZE)]

    n = 0
    while True:
        n += 1
        system('clear')
        print_board(board)
        print(f"{n}th iteration, {sum(board)} living cells.")

        neighbors = [count_live_neighbours(i, board) for i in range(SIZE)]
        board = update_board(board, neighbors)

        sleep(1)


if __name__ == '__main__':
    main()
