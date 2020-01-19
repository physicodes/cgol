import pathlib

import numpy as np
import matplotlib.pyplot as plt

FILENAME = 'sim_results.dat'


def main():
    path = pathlib.Path(FILENAME)
    file_contents = np.loadtxt(path, dtype=int, delimiter=',')
    fracs = np.array([x/10 for x in range(1, 9)])
    frac_results = np.vsplit(file_contents, 8)
    avgs = []
    stds = []
    for result in frac_results:
        avgs.append(np.mean(result, axis=0))
        stds.append(np.std(result, axis=0) / np.sqrt(result.size))
    x = np.arange(0, avgs[0].size)
    for f, y, s in zip(fracs, avgs, stds):
        plt.errorbar(x, y, s, label=str(f))
    plt.xlabel("Iterations")
    plt.ylabel("Live Cells")
    plt.legend()
    plt.show()


if __name__ == '__main__':
    main()
