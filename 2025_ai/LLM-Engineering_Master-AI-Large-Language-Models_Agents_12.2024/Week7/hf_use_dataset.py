#!/usr/bin/env python3

import dotenv
dotenv.load_dotenv("configs/local.env")

from datasets import load_dataset


ds = load_dataset("ed-donner/pricer-data")
print(ds)
