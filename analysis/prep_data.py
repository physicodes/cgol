"""Running this script reads all csv files from output by the cgol
runner, and compiles the data in a pandas DataFrame. The average and
standard deviation for the number of living cells is then calculated
over all repeats for each fraction and iteration. This condensed
DataFrame is saved as a pickle file.

Whilst it would be more efficient to calculate the average and standard
deviation before it is input to the DataFrame, this way the full dataset
is avaliable should I want to investigate variations between repeats in
the future."""

from multiprocessing import Pool
from pathlib import Path
import re

import pandas as pd
import numpy as np

DATA_DIR = 'data'  # relative path to data directory
DF_COLS = ["frac", "repeat", "iteration", "nr_alive"]


def load_file(path):
    """Takes a Path object as an argument and returns a DataFrame object.

    Reads csv file into dataframe where each row of the csv represents
    one repeat and each column corersponds to an iteration."""
    # Initialise empty DataFrame
    df = pd.DataFrame(columns=DF_COLS, dtype=int)
    # Try to extract fraction from filename
    matched = re.search('data/(.+?).csv', str(path))
    if not matched:
        print(f"Unable to parse {path}")
        return df
    frac = int(float(matched.group(1))*100)
    # Load csv data into numpy array
    data = np.loadtxt(path, dtype=int, delimiter=',')
    # Transpose each row of array and join with fraction, repeat and iteration
    # number, then append to DataFrame
    shape = (data[0].size, 1)
    frac_arr = np.full(shape, frac)
    for repeat, row in enumerate(data):
        repeat_arr = np.full(shape, repeat)
        iteration_arr = np.arange(row.size)
        iteration_arr.shape = row.shape = shape
        constructed_array = np.hstack(
                (frac_arr, repeat_arr, iteration_arr, row))
        constructed_df = pd.DataFrame(constructed_array, columns=DF_COLS)
        df = df.append(constructed_df)
    return df


def load_dir(dir_path):
    """Takes multithreaded approach to compiling all csv files from data
    directory into single DataFrame."""
    # Initialise empty DataFrame
    df = pd.DataFrame(columns=DF_COLS, dtype=int)
    # Map filepaths to loadfile function, resulting in list of
    # dataframes - one for each starting fraction
    pool = Pool()
    df_frac_list = pool.map(load_file, dir_path.iterdir())
    # Join all dataframes
    return df.append(df_frac_list)


def main():
    data_dir = Path(DATA_DIR)  # Path object from string
    df_full = load_dir(data_dir)  # Generate DataFrame from csv files
    # Calculate mean and standard deviation over repeats
    df_grouped = \
        df_full.groupby(['frac', 'iteration'])['nr_alive'].agg(['mean', 'std'])
    df_grouped.to_pickle(data_dir / "data.pickle")  # Save to pickle binary
    print(df_grouped)


if __name__ == '__main__':
    main()
