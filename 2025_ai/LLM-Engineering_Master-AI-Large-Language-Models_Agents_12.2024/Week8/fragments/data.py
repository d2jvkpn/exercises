#!/usr/bin/env python3


import pandas as pd
from datasets import load_dataset, Dataset, DatasetDict

####
dataset = load_dataset(
    "McAuley-Lab/Amazon-Reviews-2023",
    "raw_meta_Appliances",
    split="full", trust_remote_code=True,
)


#####
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


####
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


####
dataset_names = [
    "Automotive",
    "Electronics",
    "Office_Products",
    "Tools_and_Home_Improvement",
    "Cell_Phones_and_Accessories",
    "Toys_and_Games",
    "Appliances",
    "Musical_Instruments",
]

####
echo "TODO: process dataset"
