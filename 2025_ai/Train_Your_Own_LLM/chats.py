#!/usr/bin/env python3
import json

import pandas as pd


def generate_chats(df: pd.DataFrame):
    mask = df["message_id"].isin(df["parent_id"])
    leaf_df = df[~mask]
    parent_df = df[mask]

    chats = [[v] for v in leaf_df.to_dict(orient="records")]
    parent_dict = { v["message_id"]: v for v in parent_df.to_dict(orient="records") }

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
    return [{"role": v["role"], "content": v["content"]} for v in chat]

####
train = pd.read_csv("data/tokenizer/train.tsv", sep="\t")
train = train[["message_id", "parent_id", "text", "role"]]
train = train.rename(columns={"text": "content"})
train["role"] = train["role"].replace({"prompter": "user"})

train_chats = generate_chats(train)
chats = [ convert_chat(v) for v in train_chats ]

filepath = "data/tokenizer/train.chats.json"
with open(filepath, 'w', encoding='utf-8') as f:
    json.dump(chats, f, ensure_ascii=False, indent=2)
    print(f"--> saved {len(chats)} chats to {filepath}")

####
val = pd.read_csv("data/tokenizer/validation.tsv", sep="\t")
val = val[["message_id", "parent_id", "text", "role"]]
val = val.rename(columns={"text": "content"})
val["role"] = val["role"].replace({"prompter": "user"})

val_chats = generate_chats(val)
chats = [ convert_chat(v) for v in val_chats ]

filepath = "data/tokenizer/validation.chats.json"
with open(filepath, 'w', encoding='utf-8') as f:
    json.dump(chats, f, ensure_ascii=False, indent=2)
    print(f"--> saved {len(chats)} chats to {filepath}")
