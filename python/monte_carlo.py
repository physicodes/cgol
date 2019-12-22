import numpy as np
import matplotlib.pyplot as plt

import cgol


def main():
    # Run sim
    fracs = np.arange(0, 1, 0.1)
    results = [cgol.run_game(frac) for frac in fracs]
    # Plotting
    for frac, result in zip(fracs, results):
        result = np.array(result)
        plt.plot(result[:, 0], result[:, 1], label=str(frac))
    plt.legend()
    plt.show()


if __name__ == '__main__':
    main()
