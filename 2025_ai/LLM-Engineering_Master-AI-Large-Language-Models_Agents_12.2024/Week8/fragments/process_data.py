#!/usr/bin/env python3
import re, pickle

import pandas as pd
import matplotlib.pyplot as plt
import tiktoken
from datasets import Dataset
from sklearn.model_selection import train_test_split


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

# https://huggingface.co/datasets/McAuley-Lab/Amazon-Reviews-2023/tree/main/raw/meta_categories
filepath = "data/McAuley-Lab/Amazon-Reviews-2023/raw/meta_categories/meta_Appliances.jsonl"
df = pd.read_json(filepath, lines=True)

print(df.shape)
print(df.head())
print(df.iloc[0])
print(df[df["price"].isna()].head())
df['category'] = 'Appliances'

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
CHUNK_SIZE = 1000
MIN_PRICE = 0.5
MAX_PRICE = 999.49
df = df[(MIN_PRICE <= df['price']) & (df['price'] <= MAX_PRICE)]

####
MIN_TOKENS = 150 # Any less than this, and we don't have enough useful content
MAX_TOKENS = 160 # Truncate after this many tokens. Then after adding in prompt text, we will get to around 180 tokens

MIN_CHARS = 300
CEILING_CHARS = MAX_TOKENS * 7

PREFIX = "Price is $"
QUESTION = "How much does this cost to the nearest dollar?"

REMOVALS = [
    #'"Batteries Included?": "No"',
    #'"Batteries Included?": "Yes"',
    #'"Batteries Required?": "No"',
    #'"Batteries Required?": "Yes"',
    "Batteries Included?",
    "Batteries Required?",
    "By Manufacturer",
    "Item",
    "Date First",
    "Package",
    ":",
    "Number of",
    "Best Sellers",
    "Number",
    "Product ",
]

MODEL = tiktoken.encoding_for_model("gpt-4")

####
def scrub_details(details):
    """
    Clean up the details string by removing common text that doesn't add value
    """

    for k in REMOVALS:
        #details = details.replace(remove, "")
        if k in details:
            del details[k]

    return ','.join([f"{k} {v}" for k,v in details.items()])


def scrub_content(stuff):
    """
    Clean up the provided text by removing unnecessary characters and whitespace
    Also remove words that are 7+ chars and contain numbers, as these are likely irrelevant product numbers
    """

    stuff = re.sub(r'[:\[\]"{}【】\s]+', ' ', stuff).strip()
    stuff = stuff.replace(" ,", ",").replace(",,,",",").replace(",,",",")
    words = stuff.split(' ')
    select = [word for word in words if len(word)<7 or not any(char.isdigit() for char in word)]

    return " ".join(select)


def scrub(row):
    """
    Parse this datapoint and if it fits within the allowed Token range,
    then set include to True
    """

    contents = '\n'.join(row['description'])
    if contents:
        contents += '\n'

    features = '\n'.join(row['features'])
    if features:
        contents += features + '\n'

    if row['details']:
        contents += scrub_details(row['details']) + '\n'

    if len(contents) < MIN_CHARS:
        return pd.Series({'prompt': None, "token_count": None, 'include': False})

    contents = contents[:CEILING_CHARS]
    text = f"{scrub_content(row['title'])}\n{scrub_content(contents)}"
    tokens = MODEL.encode(text, disallowed_special='all')

    if len(tokens) < MIN_TOKENS:
        return pd.Series({'prompt': None, "token_count": None, 'include': False})

    tokens = tokens[:MAX_TOKENS]
    text = MODEL.decode(tokens)
    price = round(row['price'] + 1)
    prompt = f"{QUESTION}\n\n{text}\n\n{PREFIX}{price}.00"

    return pd.Series({'prompt': prompt, "token_count": len(tokens), 'include': True})


def test_prompt(prompt):
    """
    Return a prompt suitable for testing, with the actual price removed
    """
    return prompt.split(PREFIX)[0] + PREFIX


####
df[['prompt', 'token_count', 'include']] = df.apply(scrub, axis=1)
df = df[df['include']]

tokens = df['token_count']

# Plot the distribution of token counts again
plt.figure(figsize=(15, 6))
plt.title(f"Token counts: Avg {sum(tokens)/len(tokens):,.1f} and highest {max(tokens):,}\n")
plt.xlabel('Length (tokens)')
plt.ylabel('Count')
plt.hist(tokens, rwidth=0.7, color="skyblue", bins=range(0, 300, 10))
plt.show()


# Plot the distribution of prices
prices = df['price']
plt.figure(figsize=(15, 6))
plt.title(f"Prices: Avg {sum(prices)/len(prices):,.1f} and highest {max(prices):,}\n")
plt.xlabel('Price ($)')
plt.ylabel('Count')
plt.hist(prices, rwidth=0.7, color="blueviolet", bins=range(0, 1000, 10))
plt.show()

####
train, test = train_test_split(df, test_size=0.2, random_state=42)

train_prompts = 
train_prices = [item['price'] for item in train]
test_prompts = [test_prompt(item['prompt']) for item in test]
test_prices = [item['price'] for item in test]

dataset = DatasetDict({
    "train": Dataset.from_dict({
        "text": train['prompt'].to_list(),
        "price": train['price'].to_list(),
    }),
    "test": Dataset.from_dict({
        "text": train['prompt'].apply(test_prompt).to_list(),
        "price": test_prices,
    }),
})
# HF_USER = "ed-donner"
# DATASET_NAME = f"{HF_USER}/pricer-data"
# dataset.push_to_hub(DATASET_NAME, private=True)

with open('data/train.pkl', 'wb') as file:
    pickle.dump(train, file)

with open('data/test.pkl', 'wb') as file:
    pickle.dump(test, file)
