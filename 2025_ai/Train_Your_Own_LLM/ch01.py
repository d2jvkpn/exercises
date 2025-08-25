#!/usr/bin/env python3
import os
from pathlib import Path

#from data.minbpe import BasicTokenizer
from src.minbpe import RegexTokenizer

import dotenv
import regex as re
dotenv.load_dotenv("configs/local.env")

from datasets import load_dataset


#### 1. dataset
repo_id = "OpenAssistant/oasst1"
ds = load_dataset(repo_id)

train_data = ds['train'].data.to_pandas()
val_data = ds['validation'].data.to_pandas()

print("languages:", train_data['lang'].unique())

train_text = train_data[train_data['lang'] == 'en']['text']
val_text = val_data[val_data['lang'] == 'en']['text']


#### 2. tokenizer
#GPT4_SPLIT_PATTERN = r"""'(?i:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?+\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]++[\r\n]*|\s*[\r\n]|\s+(?!\S)|\s+"""
#gpt4_split_pattern = re.compile(GPT4_SPLIT_PATTERN)
# train_text = train_text.apply(lambda v: ' '.join(re.findall(gpt4_split_pattern, v)))

text_sequence = ' '.join(train_text.to_list())

tokenizer = RegexTokenizer()
tokenizer.train(text_sequence, vocab_size=1024)

vocab = tokenizer.vocab

msg = "Hello, world!"
tokens = tokenizer.encode(msg)
decoded_msg = tokenizer.decode(tokens)
print(f"--> test tokenizer: {repr(msg)} -> {tokens} -> {repr(decoded_msg)}")

max_vocab_id = list(tokenizer.vocab.keys())[-1]

tokenizer.special_tokens = {
    "<|startoftext|>": max_vocab_id + 1,
    "<|separator|>": max_vocab_id + 2,
    "<|endoftext|>": max_vocab_id + 3,
    "<|unk|>": max_vocab_id + 4,
    "<|padding|>": max_vocab_id + 5,
}

vocab_size = len(tokenizer.vocab) + len(tokenizer.special_tokens)
print(f"~~~ vocab_size: {vocab_size}")

#tokens = len(tokenizer.encode(text_sequence))
#print(f"~~~ tokens: {len(tokens)}")

output_dir = Path("data") / "tokenizer"
output_dir.mkdir(parents=True, exist_ok=True)

tokenizer.save(file_prefix=str(output_dir / "my_tokenizer"))

with open(output_dir / "train.txt", 'w', encoding='utf-8') as f:
    f.write(text_sequence)

with open(output_dir / "validation.txt", 'w', encoding='utf-8') as f:
    f.write(' '.join(val_text.to_list()))
