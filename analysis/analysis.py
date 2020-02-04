#!/usr/bin/env python
# coding: utf-8

from pathlib import Path
from collections import namedtuple
import numpy as np
import re
import matplotlib.pyplot as plt
from itertools import cycle

BOARD_WIDTH = 100
BOARD_HEIGHT = 100
BOARD_SIZE = BOARD_WIDTH * BOARD_HEIGHT

DATA_DIR = 'data'


def read_files(dir_path):
    """Read data from the runner's output directory and return a list of
    data Series objects, one for each file in the directory. Each Series
    object has the corresponding starting fraction, and average and
    standard deviation values at each iteration."""

    data_list = []
    Series = namedtuple('Series', 'frac avg std')

    data_file_paths = (p for p in dir_path.iterdir())

    for path in data_file_paths:
        # Pull fraction from filename
        matched = re.search('data/(.+?).csv', str(path))
        frac = matched.group(1)
        # Add to data_dict
        data = np.loadtxt(path, dtype=int, delimiter=',')
        data_avg = np.mean(data, axis=0) / BOARD_SIZE
        data_std = np.std(data, axis=0) / BOARD_SIZE
        data_list.append(Series(float(frac), data_avg, data_std))

    return data_list


def plot_fig(data):
    fig, ax = plt.subplots()
    colors = cycle(['C'+str(n) for n in range(9)])
    for s, c in zip(data, colors):
        frac, y, y_std = s
        x = np.arange(y.size)
        y_upper = y + y_std
        y_lower = y - y_std
        ax.plot(x, y, label=frac, color=c)
        ax.fill_between(x, y_upper, y_lower, facecolor=c, alpha=0.2)
    ax.set_xlabel("Iterations")
    ax.set_ylabel("Fraction Alive")
    ax.legend()
    fig.show()


def main():
    dir_path = Path(DATA_DIR)
    data = read_files(dir_path)
    data_full_range = (s for s in data if len(str(s.frac)) == 3)
    plot_fig(data_full_range)


if __name__ == '__main__':
    main()
