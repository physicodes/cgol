from pathlib import Path
import numpy as np
import pandas as pd
import re

DATA_DIR = 'data'


def load_files(dir_path):
    """Read raw csv files into a pandas dataframe, retaining all
    information."""

    df_cols = ["frac", "repeat", "iteration", "nr_alive"]
    df = pd.DataFrame(columns=df_cols, dtype=int)

    data_file_paths = list(dir_path.iterdir())[:2]
    nr_files = len(data_file_paths)

    for i, path in enumerate(data_file_paths):
        # Pull fraction from filename
        matched = re.search('data/(.+?).csv', str(path))
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
            constructed_df = pd.DataFrame(constructed_array, columns=df_cols)
            df = df.append(constructed_df, ignore_index=True)
        print(f"{i+1}/{nr_files} files processed.")

    return df


def main():
    path_to_data = Path(DATA_DIR)
    df_full = load_files(path_to_data)
    df_grouped = \
        df_full.groupby(['frac', 'iteration'])['nr_alive'].agg(['mean', 'std'])
    df_grouped.to_pickle(path_to_data / "data.pickle")


if __name__ == '__main__':
    main()
