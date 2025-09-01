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


def chat_to_tensors(chat, block_size, padding_token):
    result = []

    for i in range(0, len(chat) - 1, 2):
        role, content = chat[i]["role"], chat[i]["content"]
        use_msg = f"{start_of_text_token}{role}{separator_token}{content}{end_of_text_token}"
        tokens = tokenizer.encode(use_msg, allowed_special="all")

        role, content = chat[i+1]["role"], chat[i+1]["content"]
        assiatnt_msg = f"{start_of_text_token}{role}{separator_token}{content}{end_of_text_token}"
        tokens.extend(tokenizer.encode(assiatnt_msg, allowed_special="all"))

        tokens = tokens[-block_size:]

        tensor = torch.tensor(tokens)
        padded_tensor = nn.functional.pad(
            input=tensor,
            #pad=(0, block_size - len(tensor)), # right paddinig
            pad=(block_size - len(tensor), 0), # left paddinig
            value=padding_token,
        )

        result.append(padded_tensor)

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
torch.save(train_tensor, tokenizer_dir / 'train.fine-tuning.pt')
torch.save(val_tensor, tokenizer_dir / 'validation.fine-tuning.pt')
