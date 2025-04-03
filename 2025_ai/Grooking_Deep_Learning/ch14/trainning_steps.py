#!/usr/bin/env python3

import os, shelve

import polars as pl


def shelve_load(filename: str) -> dict:
    with shelve.open(filename) as db:
        return { item[0]: item[1] for item in db.items() }

shelve_path = os.path.join("data", "shelve", 's03_lstm.shelve')

db = shelve_load(shelve_path)
print(f"--> last trainning: end_at={db['end_at']}")

trainning_steps = pl.from_records(
  db["trainning_steps"],
  schema=["at", "iteration", "alpha", "batch", "min_loss", "epoch_loss"],
  orient="row",
)

print(f"""==> trainning_steps:
{trainning_steps}
""")
