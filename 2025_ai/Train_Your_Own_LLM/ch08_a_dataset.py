#!/usr/bin/env python3
from pathlib import Path
from dotenv import load_dotenv
load_dotenv(Path("configs") / "local.env")

import pandas as pd
from datasets import load_dataset
#from tqdm import tqdm

####
data_dir = Path("data") / "ch08"
data_dir.mkdir(parents=True, exist_ok=True)

#data = []
#rows = dataset["train"]["text"]
#for row in tqdm(rows):
#    data.append(row)

#rows = dataset["test"]["text"]
#for row in tqdm(rows):
#    data.append(row)

test_tsv = data_dir / "test.tsv"
if test_tsv.exists():
    train_df = pd.read_csv(data_dir / "train.tsv.gz", sep="\t")
    test_df = pd.read_csv(data_dir / "test.tsv", sep="\t")
else:
    dataset = load_dataset("atlasia/Atlaset")

    train_df = dataset["train"].to_pandas()
    test_df = dataset["test"].to_pandas()

    train_df.to_csv(data_dir / "train.tsv.gz", sep="\t", index=False)
    test_df.to_csv(data_dir / "test.tsv", sep="\t", index=False)

with open(data_dir / "AtlaSetCombined.txt", "w") as f:
    f.write(" ".join(train_df["text"].to_list()))
    f.write(" ".join(test_df["text"].to_list()))
