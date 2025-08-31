#!/usr/bin/env python3
import os, json
from pathlib import Path

import dotenv
dotenv.load_dotenv("configs/local.env")
import pandas as pd
from datasets import load_dataset


def generate_chats(df: pd.DataFrame):
    has_parent = df["message_id"].isin(df["parent_id"])
    leaves = df[~has_parent]
    parents = df[has_parent]

    chats = [[v] for v in leaves.to_dict(orient="records")]
    parent_dict = { v["message_id"]: v for v in parents.to_dict(orient="records") }

    for chat in chats:
        k = chat[0].get("parent_id")
        while not pd.isna(k) and k is not None:
            msg = parent_dict.get(k)
            if msg is None:
                break
            chat.append(msg)
            k = msg.get("parent_id")

        chat.reverse()

    return chats

def convert_chat(chat):
    if chat[0]['role'] != 'user':
        chat = chat[1:]

    chat = chat[:len(chat) // 2 * 2]

    #return [{"role": v["role"], "content": v["content"]} for v in chat]
    return chat


#### 1. load hf data
tokenizer_dir = Path("data") / "tokenizer"
tokenizer_dir.mkdir(parents=True, exist_ok=True)

repo_id = "OpenAssistant/oasst1"
ds = load_dataset(repo_id)

train_data = ds['train'].data.to_pandas()

train_data.to_csv(tokenizer_dir / 'train.dataset.tsv', sep="\t", index=False)
print(f"--> saved {tokenizer_dir / 'train.dataset.tsv'}: {train_data.shape}")

val_data = ds['validation'].data.to_pandas()
val_data.to_csv(tokenizer_dir / 'validation.dataset.tsv', sep="\t", index=False)
print(f"--> saved {tokenizer_dir / 'validation.dataset.tsv'}: {val_data.shape}")

train_data = train_data[train_data['lang'] == 'en']
val_data = val_data[val_data['lang'] == 'en']

#### 2. train data
#train_df = pd.read_csv("data/tokenizer/train.tsv", sep="\t")
train_df = train_data[["message_id", "parent_id", "text", "role"]]
train_df = train_df.rename(columns={"text": "content"})
train_df["role"] = train_df["role"].replace({"prompter": "user"})

train_chats = generate_chats(train_df)
chats = [ convert_chat(v) for v in train_chats if len(convert_chat(v)) > 0]

with open(tokenizer_dir / 'train.chats.json', 'w', encoding='utf-8') as f:
    json.dump(chats, f, ensure_ascii=False, indent=2)
    print(f"--> saved {len(chats)} chats to {tokenizer_dir / 'train.chats.json'}")


#### 3. validation data
#val = pd.read_csv("data/tokenizer/validation.tsv", sep="\t")
val_df = val_data[["message_id", "parent_id", "text", "role"]]
val_df = val_df.rename(columns={"text": "content"})
val_df["role"] = val_df["role"].replace({"prompter": "user"})

val_chats = generate_chats(val_df)
chats = [ convert_chat(v) for v in val_chats if len(convert_chat(v)) > 0]

with open(tokenizer_dir / 'validation.chats.json', 'w', encoding='utf-8') as f:
    json.dump(chats, f, ensure_ascii=False, indent=2)
    print(f"--> saved {len(chats)} chats to {tokenizer_dir / 'validation.chats.json'}")
