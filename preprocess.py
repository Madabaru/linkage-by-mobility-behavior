import requests
import xmltodict
import dask.dataframe as dd

from pathlib import Path

# Change as required
DATA_DIR = "data/"
FILE_IN = "test3.csv"
FILE_OUT = "output1.csv"


def main():
    """Main function of the program. """

    ddf = dd.read_csv(Path(DATA_DIR + FILE_IN), header=None, delimiter=",", names=[
                      "id", "speed", "timestamp", "lat", "lon", "heading"], 
                      dtype={"id": int, "lat": float, "lon": float, "heading": int}, blocksize=30)
    res = ddf.map_partitions(lambda df: df.apply(reverse_geocode, axis=1, result_type="expand"), meta={0: str, 1: str, 2: str})
    res.columns = ["street", "postcode", "state"]
    ddf = ddf.merge(res, left_index=True, right_index=True)
    ddf = ddf.drop(["lat", "lon"], axis=1)
    ddf.drop_duplicates(inplace=True)
    ddf = ddf.set_index("id")
    ddf = ddf.map_partitions(lambda df: df.sort_values(["id", "timestamp"]))
    ddf.compute(scheduler='processes')
    ddf.to_csv(Path(DATA_DIR + FILE_OUT), single_file=True)


def reverse_geocode(row):
    """Translates a given coordiante into a readable address using reverse geocoding. """
    lat = row["lat"]
    lon = row["lon"]
    url = f"https://nominatim.openstreetmap.org/reverse?lat={lat}&lon={lon}"
    response = requests.get(url)
    data = xmltodict.parse(response.content)
    addressparts = data["reversegeocode"]["addressparts"]
    street = "undefined"
    if "road" in addressparts:
        street = addressparts["road"]
    postcode = "undefined"
    if "postcode" in addressparts:
        postcode = addressparts["postcode"]
    state = "undefined"
    if "state" in addressparts:
        state = addressparts["state"]
    return street, postcode, state



if __name__ == "__main__":
    main()
