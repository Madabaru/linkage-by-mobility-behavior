import dask.dataframe as dd
import numpy as np
import pandas as pd

from pathlib import Path

# Change as required
DATA_DIR = "data/"
FILE_IN = "sample_small.csv"
FILE_OUT = "output.csv"

BLOCK_SIZE = 256
NUM_SAMPLES = 20


def main():
    """Main function of the program. """


    ddf = dd.read_csv(Path(DATA_DIR + FILE_IN), header=None, delimiter=",", names=[
                      "id", "speed", "timestamp", "lat", "lon", "heading"], 
                      dtype={"speed": int, "heading": int})
                      # blocksize=BLOCK_SIZE) # dtype={"id": str, "lat": float, "lon": float, "heading": int}

    sampled = np.random.choice(ddf["id"], size=NUM_SAMPLES, replace=False)
    ddf_selected = ddf[ddf["id"].isin(sampled)]

   
    # res = ddf.map_partitions(lambda df: df.apply(reverse_geocode, axis=1, result_type="expand"), meta={0: str, 1: str, 2: str})
    # res.columns = ["street", "postcode", "state"]
    # ddf = ddf.merge(res, left_index=True, right_index=True)
    # ddf = ddf.drop(["lat", "lon"], axis=1)
    # ddf.drop_duplicates(inplace=True)
    # ddf = ddf.set_index("id")
    # ddf = ddf.map_partitions(lambda df: df.sort_values(["id", "timestamp"]))

    ddf_selected = ddf_selected.sort_values("id")
    ddf_selected.compute(scheduler='processes')
    ddf_selected.to_csv(Path(DATA_DIR + FILE_OUT), single_file=True, index=False)



if __name__ == "__main__":
    main()
