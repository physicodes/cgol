from pathlib import Path
import numpy as np
import pandas as pd
import re
from multiprocessing import Pool

DATA_DIR = 'data'
DF_COLS = ["frac", "repeat", "iteration", "nr_alive"]


def load_file(path):
    df = pd.DataFrame(columns=DF_COLS, dtype=int)
    # Pull fraction from filename
    matched = re.search('data/(.+?).csv', str(path))
    if not matched:
        print(f"Unable to parse {path}")
        return df
    frac = int(float(matched.group(1))*100)
    # Add to data_dict
    data = np.loadtxt(path, dtype=int, delimiter=',')
    for repeat, row in enumerate(data):
        shape = row.size, 1
        afrac = np.full(shape, frac)
        arepeat = np.full(shape, repeat)
        aiteration = np.arange(row.size)
        aiteration.shape = shape
        row.shape = shape
        constructed_array = np.hstack((afrac, arepeat, aiteration, row))
        constructed_df = pd.DataFrame(constructed_array, columns=DF_COLS)
        df = df.append(constructed_df)
    return df


def load_files(dir_path):
    """Read raw csv files into a pandas dataframe, retaining all
    information."""

    df = pd.DataFrame(columns=DF_COLS, dtype=int)

    data_file_paths = list(dir_path.iterdir())

    pool = Pool()
    df_frac_list = pool.map(load_file, data_file_paths)
    df = df.append(df_frac_list)

    return df


def main():
    path_to_data = Path(DATA_DIR)
    df_full = load_files(path_to_data)
    print(df_full)
    df_grouped = \
        df_full.groupby(['frac', 'iteration'])['nr_alive'].agg(['mean', 'std'])
    df_grouped.to_pickle(path_to_data / "data.pickle")
    print(df_grouped)


if __name__ == '__main__':
    main()
