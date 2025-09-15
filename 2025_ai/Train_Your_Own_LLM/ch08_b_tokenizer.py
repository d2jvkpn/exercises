#!/usr/bin/env python3
from pathlib import Path

from src.minbpe import RegexTokenizer as Tokenizer

import numpy as np

####
data_dir = Path("data") / "ch08"
number_of_characters_to_read = 10_000_000

with open(data_dir / "AtlaSetCombined.txt", "r") as f:
    text_sequence = f.read(number_of_characters_to_read)

tokenizer = Tokenizer()
tokenizer.train(text_sequence, vocab_size=16_384)

tokenizer.add_special_tokens([
    "<|startoftext|>",
    "<|separator|>",
    "<|endoftext|>",
    "<|unk|>",
    "<|padding|>",
])

tokenizer.save(file_prefix=str(data_dir / "darija_tokenizer"))

####
tokenizer = Tokenizer()
tokenizer.load(model_file=data_dir / "darija_tokenizer.model")

encoded_text_sequence = []
batch_size = 100_000_000

####
with open(data_dir / "AtlaSetCombined.txt", "r") as f:
    while True:
        chunk = f.read(batch_size)
        if not chunk:
            break

        batch_tokens = tokenizer.encode(chunk)
        encoded_text_sequence.extend(batch_tokens)
        print(f"Processed {len(encoded_text_sequence)} tokens so far.")

print(f"Total tokens: {len(encoded_text_sequence)}")

encoded_atlaset = np.array(encoded_text_sequence, dtype=np.int64)
np.save(data_dir / "encoded_atlaset.npy", encoded_atlaset)

# Free up memory
#del encoded_text_sequence
