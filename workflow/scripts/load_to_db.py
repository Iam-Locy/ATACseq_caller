import pandas as pd
import duckdb
import os

COLUMNS = ["chrom", "start", "end", "name", "score", 
           "strand", "signalValue", "pValue", "qValue", "peak"]


dfs = []
for root, dirs, files in os.walk("results/genrich_peaks"):
    for file in files:
        tmp = pd.read_csv(
            f"{root}/{file}",
            sep='\t',
            header=None,
            names=COLUMNS,
            dtype={
                "chrom": str,
                "start": str,
                "end": str,
                "name": str,
                "score": int, 
                "strand": str,
                "signalValue": float,
                "pValue": float,
                "qValue": float,
                "peak": int
            }
        )
        tmp.insert(0, 'sample', file.replace(".narrowPeak", ""))
        dfs.append(tmp)

all_peaks = pd.concat(dfs, ignore_index=True)

with duckdb.connect("results/peaks.db") as con:
    con.sql("CREATE TABLE peaks AS (SELECT * FROM all_peaks);")
    con.sql("SELECT DISTINCT sample FROM peaks").show()