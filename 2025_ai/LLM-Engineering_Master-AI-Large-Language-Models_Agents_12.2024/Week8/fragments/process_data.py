#!/usr/bin/env python3
import re, random, pickle
random.seed(42)

import pandas as pd
import tiktoken
from datasets import Dataset
from sklearn.model_selection import train_test_split


df = pd.read_json(
    "data/McAuley-Lab/Amazon-Reviews-2023/raw/meta_categories/meta_Appliances.jsonl",
    lines=True,
)

print(df.shape)
print(df.head())
print(df.iloc[0])
print(df[df["price"].isna()].head())
df['category'] = 'Appliances'

####
df = df.dropna(subset=['price'])
print(df.shape)

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

    return ','.join([f"{k} {v}" for k,v in x.items()])


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

self.prompt.split(self.PREFIX)[0] + self.PREFIX

train, test = train_test_split(
    df[df['include']],
    test_size=0.2, random_state=42,
)

train_prompts = [item['prompt'] for item in train]
train_prices = [item['price'] for item in train]
test_prompts = [test_prompt(item['prompt']) for item in test]
test_prices = [item['price'] for item in test]

dataset = DatasetDict({
    "train": Dataset.from_dict({"text": train_prompts, "price": train_prices}),
    "test": Dataset.from_dict({"text": test_prompts, "price": test_prices}),
})
# HF_USER = "ed-donner"
# DATASET_NAME = f"{HF_USER}/pricer-data"
# dataset.push_to_hub(DATASET_NAME, private=True)

with open('data/train.pkl', 'wb') as file:
    pickle.dump(train, file)

with open('data/test.pkl', 'wb') as file:
    pickle.dump(test, file)
