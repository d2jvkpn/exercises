#!/usr/bin/env python3
from pathlib import Path
from dotenv import load_dotenv
load_dotenv(Path("configs") / "local.env")

import transformers
from transformers import DistilBertTokenizer


tokenizer = DistilBertTokenizer.from_pretrained("distilbert-base-uncased")

sents = [
    "I love chess",
    "I love soccer and material arts",
]

inputs = tokenizer.encode_plus(
    *sents,
    add_special_tokens=True,
    max_length=20,
    padding="max_length",
    truncation=True,
    return_attention_mask=True,
    return_token_type_ids=True,
)

print(
    f"sentences: {sents}\n"
    f"input_ids: {inputs['input_ids']}\n"
    f"attention_mask: {inputs['attention_mask']}\n"
    f"token_type_ids: {inputs['token_type_ids']}\n"
)
