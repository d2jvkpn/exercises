#!/usr/bin/env python3
from pathlib import Path

from dotenv import load_dotenv
from transformers import AutoTokenizer


###
load_dotenv(dotenv_path=Path("configs") / "local.env")

####
tokenizer = AutoTokenizer.from_pretrained(
    "data/models/deepseek-ai/deepseek-coder-1.3b-instruct.model",
    trust_remote_code=True,
)

print(len(tokenizer.vocab), tokenizer.vocab_size, tokenizer.model_max_length)
# 32022 32000 16384


####
text = "I am excited to show Tokenizers in action to my LLM engineers."

tokens = tokenizer.encode(text)
output = [32013,
 40,
 604,
 9219,
 276,
 1294,
 323,
 4163,
 18925,
 279,
 3423,
 276,
 597,
 27480,
 44,
 24376,
 13]

decoded = tokenizer.decode(tokens)
# '<｜begin▁of▁sentence｜>I am excited to show Tokenizers in action to my LLM engineers.'

batch_decoded = tokenizer.batch_decode(tokens)
# ['<｜begin▁of▁sentence｜>', 'I', ' am', ' excited', ' to', ' show', ' T', 'oken', 'izers', ' in', ' action', ' to', ' my', ' LL', 'M', ' engineers', '.']


print(tokenizer.get_added_vocab())
output = {'õ': 32000,
 '÷': 32001,
 'Á': 32002,
 'ý': 32003,
 'À': 32004,
 'ÿ': 32005,
 'ø': 32006,
 'ú': 32007,
 'þ': 32008,
 'ü': 32009,
 'ù': 32010,
 'ö': 32011,
 'û': 32012,
 '<｜begin▁of▁sentence｜>': 32013,
 '<｜end▁of▁sentence｜>': 32014,
 '<｜fim▁hole｜>': 32015,
 '<｜fim▁begin｜>': 32016,
 '<｜fim▁end｜>': 32017,
 '<pad>': 32018,
 '<|User|>': 32019,
 '<|Assistant|>': 32020,
 '<|EOT|>': 32021}

print(tokenizer.encode("你好"), tokenizer.encode("Hello"))
# [32013, 1367, 1248] [32013, 17535]

####
messages = [
    {"role": "system", "content": "You are a helpful assitant."},
    {"role": "user", "content": "Tell me a light-hearted joke from a room of Data Scientists."},
]

prompt = tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
# '<｜begin▁of▁sentence｜>You are a helpful assitant.### Instruction:\nTell me a light-hearted joke from a room of Data Scientists\n### Response:\n'
