#!/usr/bin/env python3
import os, json
from pathlib import Path

#from data.minbpe import BasicTokenizer as Tokenizer
from src.minbpe import RegexTokenizer as Tokenizer

import dotenv
dotenv.load_dotenv("configs/local.env")
import torch


#### 1.
tokenizer_dir = Path("data") / "tokenizer"

with open(tokenizer_dir / "train.chats.json") as f:
    chats = json.load(f)

    msgs = []
    for chat in chats:
        for msg in chat:
            msgs.append(msg['content'])

    train_text = " ".join(msgs)

with open(tokenizer_dir / "validation.chats.json") as f:
    chats = json.load(f)

    msgs = []
    for chat in chats:
        for msg in chat:
            msgs.append(msg['content'])

    val_text = " ".join(msgs)

#### 2.
tokenizer = Tokenizer()
tokenizer.train(train_text + " " + val_text, vocab_size=1024)

#### 3.
msg = "Hello, world!"
tokens = tokenizer.encode(msg)
decoded_msg = tokenizer.decode(tokens)
print(f"--> tokenizer: {repr(msg)} -> {tokens} -> {repr(decoded_msg)}")

tokenizer.add_special_tokens([
    "<|startoftext|>",
    "<|separator|>",
    "<|endoftext|>",
    "<|unk|>",
    "<|padding|>",
])

print(f"--> vocab_size: {len(tokenizer.vocab):_}")
tokenizer.save(file_prefix=str(tokenizer_dir / "tokenizer"))

#### 4.
# tokenizer = Tokenizer()
# tokenizer.load(model_file=str(tokenizer_dir / "tokenizer.model"))
train_tokens = tokenizer.encode(train_text, allowed_special="all")
val_tokens = tokenizer.encode(val_text, allowed_special="all")

print(f"--> tokens: train_tokens={len(train_tokens):_}, val_tokens={len(val_tokens):_}")

train_pt = torch.tensor(train_tokens, dtype=torch.long)
val_pt = torch.tensor(val_tokens, dtype=torch.long)

torch.save(train_pt, tokenizer_dir / 'train.tokens.pt')
torch.save(val_pt, tokenizer_dir / 'validation.tokens.pt')

#train_pt = torch.load(tokenizer_dir /'train.tokens.pt')
