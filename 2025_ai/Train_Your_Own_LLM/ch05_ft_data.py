#!/usr/bin/env python3
import json
from pathlib import Path
from typing import Tuple
from itertools import chain

from src.minbpe import RegexTokenizer as Tokenizer
from src.gpt import GPTLanguageModel

import torch
from torch import nn


tokenizer_dir = Path("data") / "tokenizer"

tokenizer = Tokenizer()
tokenizer.load(model_file=str(tokenizer_dir / "tokenizer.model"))

start_of_text_token = "<|startoftext|>"
separator_token = "<|separator|>"
end_of_text_token = "<|endoftext|>"
unk_token = "<|unk|>"
#padding_token = "<|padding|>"


def msg2tokens(msg):
    seq = f"{start_of_text_token}{msg['role']}{separator_token}{msg['content']}{end_of_text_token}"
    return tokenizer.encode(seq, allowed_special="all")

def chat_to_tensors(chat, block_size, padding_token):
    combined = []

    for i in range(0, len(chat) - 1, 2):
        tokens = msg2tokens(chat[i]) + msg2tokens(chat[i+1])
        combined.append(tokens[-block_size:])

    current, result = [], []
    for seq in combined:
        if len(current) + len(seq) <= block_size:
            current.extend(seq)
        else:
            if len(current) > 0:
                result.append(current)
            current = seq.copy()

    # Add the last block if it's not empty
    if current:
        result.append(current)

    for i in range(len(result)):
        tensor = torch.tensor(result[i])

        padded_tensor = nn.functional.pad(
            input=tensor,
            pad=(0, block_size - len(tensor)), # right paddinig
            #pad=(block_size - len(tensor), 0), # left paddinig
            value=padding_token,
        )

        result[i] = padded_tensor

    return result


with open(tokenizer_dir / "train.chats.json", 'r') as f:
    train_chats = json.load(f)

with open(tokenizer_dir / "validation.chats.json", 'r') as f:
    val_chats = json.load(f)


block_size = 256 # ch02: 256, ch03: 512
padding_token = -100

train_tensors = [chat_to_tensors(v, block_size, padding_token) for v in train_chats]
val_tensors = [chat_to_tensors(v, block_size, padding_token) for v in val_chats]

train_tensor = torch.stack(list(chain.from_iterable(train_tensors)))
val_tensor = torch.stack(list(chain.from_iterable(val_tensors)))

print(f"--> Train: {train_tensor.shape}\n    Validation: {val_tensor.shape}")
torch.save(train_tensor, tokenizer_dir / 'ch05_train.ft.pt')
torch.save(val_tensor, tokenizer_dir / 'ch05_validation.ft.pt')