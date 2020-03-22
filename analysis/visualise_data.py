from bokeh.io import output_file
from bokeh.plotting import figure, show
import pandas as pd

BOARD_WIDTH = 100
BOARD_HEIGHT = 100
BOARD_SIZE = BOARD_WIDTH * BOARD_HEIGHT

PROCESSED_DATA = 'data/data.pickle'


def main():
    df = pd.read_pickle(PROCESSED_DATA)  # Load data

    output_file('test.html', title='Empty Figure')
    fig = figure()
    show(fig)


if __name__ == '__main__':
    main()
