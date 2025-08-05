#!/usr/bin/env python3
import os

import dotenv
import pandas as pd
from huggingface_hub import login, snapshot_download
from datasets import load_dataset, load_from_disk, Dataset, DatasetDict
import matplotlib.pyplot as plt

dotenv.load_dotenv("configs/local.env")

hf_token = os.environ["HF_TOKEN"]
login(hf_token, add_to_git_credential=True)


#iconv -f us-ascii -t utf-8 \
#  data/McAuley-Lab/Amazon-Reviews-2023/raw/meta_categories/meta_Appliances.jsonl \
#  -o data/McAuley-Lab/Amazon-Reviews-2023/raw/meta_categories/meta_Appliances.utf-8.jsonl

#### failed
#try:
#    dataset = load_dataset(
#        "json",
#        data_files=["data/McAuley-Lab/Amazon-Reviews-2023/raw/meta_categories/meta_Appliances.utf-8.jsonl"],
#    )
#except Exception as e:
#    import traceback
#    traceback.print_exc()


df = pd.read_json(
    "data/McAuley-Lab/Amazon-Reviews-2023/raw/meta_categories/meta_Appliances.jsonl",
    lines=True,
)
print(df.shape)
print(df.head())
print(df.iloc[0])
print(df[df["price"].isna()].head())

df = df.dropna(subset=['price'])
print(df.shape)


prices = df['price']

contents = df['title'] + "; " + \
    df['description'].apply(lambda x: " ".join(x) if isinstance(x, list) else "") + "; " + \
    df['features'].apply(lambda x: " ".join(x) if isinstance(x, list) else "") + "; " + \
    df['features'].apply(lambda x: str(x))

lengths = contents.apply(lambda x: len(x))

plt.figure(figsize=(15, 6))
plt.title(f"Length: avg={lengths.sum()/lengths.shape[0]:_.0f}, highest={lengths.max():_}")
plt.xlabel("Length (chars)")
plt.ylabel("Count")
plt.hist(lengths, rwidth=0.7, color="lightblue", bins=range(0, 6000, 100))
plt.show()


plt.figure(figsize=(15, 6))
plt.title(f"Prices: avg={prices.sum()/prices.shape[0]:_.0f}, highest={prices.max()}")
plt.xlabel("Prices ($)")
plt.ylabel("Count")
plt.hist(prices, rwidth=0.7, color="orange", bins=range(0, 500, 10))
plt.show()

# df = df.drop(columns=["Best Sellers Rank"], errors="ignore")  # 删除问题字段
dataset = Dataset.from_pandas(df)

print(f"Number of Appliances: {len(dataset):,}")


snapshot_download(repo_id="mistralai/Mistral-7B-v0.3", cache_dir="./data", resume_download=True)

snapshot_download(
    repo_id="mistralai/Mistral-7B-Instruct-v0.3",
    cache_dir="./data", resume_download=True,
)
