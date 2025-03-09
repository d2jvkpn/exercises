#!/usr/bin/env python3

from os import path
import json
from collections import Counter

import polars as pl


with open(path.join("data", "word2index.json"), 'r') as f:
    word2index = json.load(f)

print(word2index)

weights_0_1 = pl.read_csv(path.join("data", "weights_0_1.tsv"), separator="\t").to_numpy()
print(weights_0_1)

weights_1_2 = pl.read_csv(path.join("data", "weights_1_2.tsv"), separator="\t").to_numpy()
print(weights_1_2)
